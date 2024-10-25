pub fn print_config(config: Configuration) -> String {
    let mut document = XmlDocument::new();

    let mut filter_options = XmlElement::open("filteroptions");

    for account in config.accounts {
        let evolution_id = account.evolution_id.expect("evolution_id must be set");

        let mut rule_set = XmlElement::open("ruleset");

        for message_filter in account.message_filters {
            let mut rule = XmlElement::open_attr(
                "rule",
                "enabled=\"true\" grouping=\"any\" source=\"incoming\"",
            );

            rule.append_child({
                let mut title = XmlElement::open("title");
                title.append(message_filter.title);
                title
            });

            let mut part_set = XmlElement::open("partset");

            for condition in message_filter.conditions {
                match condition {
                    Condition::Contains(contains) => {
                        let field = helpers::format_field(&contains.field);

                        for needle in contains.values {
                            let mut xml_part =
                                XmlElement::open_attr("part", format!("name=\"{}\"", field));

                            xml_part.append_child(XmlElement::open_attr(
                                "value",
                                "name=\"sender-type\" type=\"option\" value=\"contains\"",
                            ));

                            let mut value = XmlElement::open_attr(
                                "value",
                                format!("name=\"{}\" type=\"string\" allow-empty=\"false\"", field),
                            );

                            value.append_child({
                                let mut string = XmlElement::open("string");
                                string.append(needle);
                                string
                            });

                            xml_part.append_child(value);
                            part_set.append_child(xml_part);
                        }
                    }
                    Condition::EndsWith(ends_with) => {
                        let field = helpers::format_field(&ends_with.field);

                        for suffix in ends_with.values {
                            let mut xml_part =
                                XmlElement::open_attr("part", format!("name=\"{}\"", field));

                            xml_part.append_child(XmlElement::open_attr(
                                "value",
                                "name=\"sender-type\" type=\"option\" value=\"ends with\"",
                            ));

                            let mut value = XmlElement::open_attr(
                                "value",
                                format!("name=\"{}\" type=\"string\" allow-empty=\"false\"", field),
                            );

                            value.append_child({
                                let mut string = XmlElement::open("string");
                                string.append(suffix);
                                string
                            });

                            xml_part.append_child(value);
                            part_set.append_child(xml_part);
                        }
                    }
                }
            }

            rule.append_child(part_set);

            let mut action_set = XmlElement::open("actionset");

            let mut part = XmlElement::open_attr("part", "name=\"move-to-folder\"");

            let mut value = XmlElement::open_attr("value", "name=\"folder\" type=\"folder\"");
            value.append_child(XmlElement::open_attr(
                "folder",
                format!(
                    "uri=\"folder://{}/{}\"",
                    evolution_id, message_filter.move_to
                ),
            ));
            part.append_child(value);
            action_set.append_child(part);

            rule.append_child(action_set);
            rule_set.append_child(rule);
        }

        filter_options.append_child(rule_set);
    }

    document.append_element(filter_options);
    document.finish()
}

mod helpers {

    pub fn format_field(field: &str) -> String {
        return if field == "from" {
            "sender".to_owned()
        } else {
            field.to_owned()
        };
    }
}

use crate::{
    configuration::{Condition, Configuration},
    xml::{XmlDocument, XmlElement},
};
