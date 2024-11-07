#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("evolution_id is missing")]
    MissingEvolutionId,
}

pub fn print_config(config: Configuration) -> Result<String> {
    let mut document = XmlDocument::new();

    let mut filter_options = XmlElementBuilder::new("filteroptions");

    for account in config.accounts {
        let evolution_id = account.evolution_id.ok_or(Error::MissingEvolutionId)?;

        let mut rule_set = XmlElementBuilder::new("ruleset");

        for message_filter in account.message_filters {
            let mut rule = XmlElementBuilder::new("rule");
            rule.append_attr("enabled", "true")
                .append_attr("grouping", "any")
                .append_attr("source", "incoming");

            rule.append_child({
                let mut title = XmlTextElementBuilder::new("title");
                title.append_text(message_filter.title);
                title.build()
            });

            let mut part_set = XmlElementBuilder::new("partset");

            for condition in message_filter.when {
                match condition {
                    When::Contains(contains) => {
                        let field = helpers::format_field(&contains.field);

                        for needle in contains.values {
                            let mut xml_part = XmlElementBuilder::new("part");
                            xml_part.append_attr("name", &field);

                            xml_part.append_child({
                                let mut value = XmlElementBuilder::new("value");
                                value
                                    .append_attr("name", format!("{field}-type"))
                                    .append_attr("type", "option")
                                    .append_attr("value", "contains");
                                value.build()
                            });

                            xml_part.append_child({
                                let mut value = XmlElementBuilder::new("value");
                                value
                                    .append_attr("name", &field)
                                    .append_attr("type", "string")
                                    .append_attr("allow-empty", "false");
                                value.append_child({
                                    let mut string = XmlTextElementBuilder::new("string");
                                    string.append_text(needle);
                                    string.build()
                                });
                                value.build()
                            });

                            part_set.append_child(xml_part.build());
                        }
                    }
                    When::EndsWith(ends_with) => {
                        let field = helpers::format_field(&ends_with.field);

                        for suffix in ends_with.values {
                            let mut xml_part = XmlElementBuilder::new("part");
                            xml_part.append_attr("name", &field);

                            xml_part.append_child({
                                let mut value = XmlElementBuilder::new("value");
                                value
                                    .append_attr("name", format!("{field}-type"))
                                    .append_attr("type", "option")
                                    .append_attr("value", "ends with");
                                value.build()
                            });

                            xml_part.append_child({
                                let mut value = XmlElementBuilder::new("value");
                                value
                                    .append_attr("name", &field)
                                    .append_attr("type", "string")
                                    .append_attr("allow-empty", "false");
                                value.append_child({
                                    let mut string = XmlTextElementBuilder::new("string");
                                    string.append_text(suffix);
                                    string.build()
                                });
                                value.build()
                            });

                            part_set.append_child(xml_part.build());
                        }
                    }
                }
            }

            rule.append_child(part_set.build());

            let mut action_set = XmlElementBuilder::new("actionset");

            for action in message_filter.then {
                match action {
                    Then::MoveTo(move_to) => {
                        let mut part = XmlElementBuilder::new("part");
                        part.append_attr("name", "move-to-folder");

                        part.append_child({
                            let mut value = XmlElementBuilder::new("value");
                            value
                                .append_attr("name", "folder")
                                .append_attr("type", "folder");
                            value.append_child({
                                let mut folder = XmlElementBuilder::new("folder");
                                folder.append_attr(
                                    "uri",
                                    helpers::format_folder(&evolution_id, &move_to.folder),
                                );
                                folder.build()
                            });
                            value.build()
                        });
                        action_set.append_child(part.build());
                    }
                }
            }

            rule.append_child(action_set.build());
            rule_set.append_child(rule.build());
        }

        filter_options.append_child(rule_set.build());
    }

    document.append_element(filter_options.build());

    Ok(document.to_string())
}

mod helpers {

    pub fn format_field(field: &Field) -> String {
        match field {
            Field::From => "sender".to_owned(),
        }
    }

    pub fn format_folder(account: &str, folder: &str) -> String {
        format!("folder://{account}/{folder}")
    }

    use crate::configuration::Field;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_print_config() {
        let config = Configuration {
            accounts: vec![Account {
                evolution_id: Some("evolution".to_owned()),
                thunderbird_id: Some("thunderbird".to_owned()),
                message_filters: vec![
                    MessageFilter {
                        title: "DigitalOcean".to_owned(),
                        when: vec![When::EndsWith(EndsWith {
                            field: Field::From,
                            values: vec!["@digitalocean.com".to_owned()],
                        })],
                        then: vec![Then::MoveTo(MoveTo {
                            folder: "do".to_owned(),
                        })],
                    },
                    MessageFilter {
                        title: "Amazon".to_owned(),
                        when: vec![When::Contains(Contains {
                            field: Field::From,
                            values: vec!["@amazon.".to_owned()],
                        })],
                        then: vec![Then::MoveTo(MoveTo {
                            folder: "amzn".to_owned(),
                        })],
                    },
                ],
            }],
        };

        assert_eq!(
            print_config(config).unwrap(),
            vec![
                "<?xml version=\"1.0\"?>",
                "<filteroptions>",
                "<ruleset>",
                "<rule enabled=\"true\" grouping=\"any\" source=\"incoming\">",
                "<title>DigitalOcean</title>",
                "<partset>",
                "<part name=\"sender\">",
                "<value name=\"sender-type\" type=\"option\" value=\"ends with\"></value>",
                "<value name=\"sender\" type=\"string\" allow-empty=\"false\">",
                "<string>@digitalocean.com</string>",
                "</value>",
                "</part>",
                "</partset>",
                "<actionset>",
                "<part name=\"move-to-folder\">",
                "<value name=\"folder\" type=\"folder\">",
                "<folder uri=\"folder://evolution/do\">",
                "</folder>",
                "</value>",
                "</part>",
                "</actionset>",
                "</rule>",
                "<rule enabled=\"true\" grouping=\"any\" source=\"incoming\">",
                "<title>Amazon</title>",
                "<partset>",
                "<part name=\"sender\">",
                "<value name=\"sender-type\" type=\"option\" value=\"contains\"></value>",
                "<value name=\"sender\" type=\"string\" allow-empty=\"false\">",
                "<string>@amazon.</string>",
                "</value>",
                "</part>",
                "</partset>",
                "<actionset>",
                "<part name=\"move-to-folder\">",
                "<value name=\"folder\" type=\"folder\">",
                "<folder uri=\"folder://evolution/amzn\">",
                "</folder>",
                "</value>",
                "</part>",
                "</actionset>",
                "</rule>",
                "</ruleset>",
                "</filteroptions>"
            ]
            .join("")
        );
    }

    use super::*;
    use crate::configuration::{Account, Contains, EndsWith, Field, MessageFilter, MoveTo, When};
}

use crate::{
    configuration::{Configuration, Then, When},
    xml::{XmlDocument, XmlElementBuilder, XmlTextElementBuilder},
    Result,
};
