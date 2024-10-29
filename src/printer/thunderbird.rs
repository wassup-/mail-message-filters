#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("thunderbird_id is missing")]
    MissingThunderbirdId,
}

pub fn print_config(config: Configuration) -> Result<String> {
    let mut document = DatDocument::new(9, false);

    for account in config.accounts {
        let thunderbird_id = account.thunderbird_id.ok_or(Error::MissingThunderbirdId)?;

        for message_filter in account.message_filters {
            for action in message_filter.actions {
                match action {
                    Action::MoveTo(move_to) => {
                        let move_to =
                            helpers::format_folder("imap", &thunderbird_id, &move_to.folder);

                        helpers::append_filter(
                            &mut document,
                            &message_filter.title,
                            &move_to,
                            &message_filter.conditions,
                        )
                    }
                }
            }
        }
    }

    Ok(document.to_string())
}

mod helpers {

    pub fn append_filter(
        doc: &mut DatDocument,
        name: &str,
        move_to: &str,
        conditions: &[Condition],
    ) {
        doc.append("name", quote(name));
        doc.append("enabled", quote("yes"));
        doc.append("type", quote("17"));
        doc.append("action", quote("Move to folder"));
        doc.append("actionValue", quote(move_to));
        doc.append("condition", format_condition(conditions));
    }

    fn format_condition(conditions: &[Condition]) -> String {
        let inner: Vec<String> = conditions
            .iter()
            .map(|cond| match cond {
                Condition::Contains(cond) => {
                    let prefix: &'static str = if cond.values.len() == 1 { "AND" } else { "OR" };
                    let field = format_field(&cond.field);
                    cond.values
                        .iter()
                        .map(move |value| format!("{} ({},contains,{})", prefix, field, value))
                        .collect::<Vec<_>>()
                        .join(" ")
                }
                Condition::EndsWith(cond) => {
                    let prefix: &'static str = if cond.values.len() == 1 { "AND" } else { "OR" };
                    let field = format_field(&cond.field);
                    cond.values
                        .iter()
                        .map(move |value| format!("{} ({},ends with,{})", prefix, field, value))
                        .collect::<Vec<_>>()
                        .join(" ")
                }
            })
            .collect();

        inner.join(" ")
    }

    pub fn format_folder(scheme: &str, account: &str, folder: &str) -> String {
        assert!(!scheme.is_empty());
        assert!(!account.is_empty());
        assert!(!folder.is_empty());

        format!("{scheme}://{account}/{folder}")
    }

    pub fn format_field(field: &Field) -> String {
        match field {
            Field::From => "from".to_owned(),
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_format_one_condition() {
            assert_eq!(
                format_condition(&[Condition::EndsWith(EndsWith {
                    field: Field::From,
                    values: vec!["@example.com".to_owned()]
                })]),
                "AND (from,ends with,@example.com)"
            );
        }

        #[test]
        fn test_format_multiple_conditions() {
            assert_eq!(
                format_condition(&[Condition::EndsWith(EndsWith {
                    field: Field::From,
                    values: vec!["@example.com".to_owned(), "@test.com".to_owned()]
                })]),
                "OR (from,ends with,@example.com) OR (from,ends with,@test.com)"
            );
        }

        use super::*;
        use crate::configuration::{EndsWith, Field};
    }

    use crate::{
        configuration::{Condition, Field},
        dat::DatDocument,
        util::quote,
    };
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
                        conditions: vec![Condition::EndsWith(EndsWith {
                            field: Field::From,
                            values: vec!["@digitalocean.com".to_owned()],
                        })],
                        actions: vec![Action::MoveTo(MoveTo {
                            folder: "do".to_owned(),
                        })],
                    },
                    MessageFilter {
                        title: "Amazon".to_owned(),
                        conditions: vec![Condition::Contains(Contains {
                            field: Field::From,
                            values: vec!["@amazon.".to_owned()],
                        })],
                        actions: vec![Action::MoveTo(MoveTo {
                            folder: "amzn".to_owned(),
                        })],
                    },
                ],
            }],
        };

        assert_eq!(
            print_config(config).unwrap(),
            vec![
                "version=\"9\"",
                "logging=\"no\"",
                "name=\"DigitalOcean\"",
                "enabled=\"yes\"",
                "type=\"17\"",
                "action=\"Move to folder\"",
                "actionValue=\"imap://thunderbird/do\"",
                "condition=AND (from,ends with,@digitalocean.com)",
                "name=\"Amazon\"",
                "enabled=\"yes\"",
                "type=\"17\"",
                "action=\"Move to folder\"",
                "actionValue=\"imap://thunderbird/amzn\"",
                "condition=AND (from,contains,@amazon.)"
            ]
            .join("\n")
        );
    }

    use super::*;
    use crate::configuration::{
        Account, Action, Condition, Contains, EndsWith, Field, MessageFilter, MoveTo,
    };
}

use crate::{
    configuration::{Action, Configuration},
    dat::DatDocument,
    Result,
};
