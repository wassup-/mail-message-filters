pub fn print_config(config: Configuration) -> String {
    let mut document = DatDocument::new();

    for account in config.accounts {
        let thunderbird_id = account.thunderbird_id.expect("thunderbird_id must be set");

        for message_filter in account.message_filters {
            let move_to = format!("imap://{}/{}", thunderbird_id, message_filter.move_to);

            helpers::append_filter(
                &mut document,
                message_filter.title,
                move_to,
                &message_filter.conditions,
            )
        }
    }

    document.to_string()
}

mod helpers {

    pub fn append_filter(
        doc: &mut DatDocument,
        name: String,
        move_to: String,
        conditions: &[Condition],
    ) {
        doc.append_str("name", name);
        doc.append_str("enabled", "yes");
        doc.append_str("type", "17");
        doc.append_str("action", "Move to folder");
        doc.append_str("actionValue", move_to);
        doc.append("condition", format_condition(conditions));
    }

    fn format_condition(conditions: &[Condition]) -> String {
        let inner: Vec<String> = conditions
            .iter()
            .map(|cond| match cond {
                Condition::Contains(cond) => {
                    let prefix: &'static str = if cond.values.len() == 1 { "AND" } else { "OR" };
                    let iter = cond.values.iter().map(move |value| {
                        format!("{} ({},contains,{})", prefix, cond.field, value)
                    });
                    iter_join(iter, " ")
                }
                Condition::EndsWith(cond) => {
                    let prefix: &'static str = if cond.values.len() == 1 { "AND" } else { "OR" };
                    let iter = cond.values.iter().map(move |value| {
                        format!("{} ({},ends with,{})", prefix, cond.field, value)
                    });
                    iter_join(iter, " ")
                }
            })
            .collect();

        inner.join(" ")
    }

    fn iter_join<I, S>(mut i: I, sep: &str) -> String
    where
        I: Iterator<Item = S>,
        S: std::fmt::Display,
    {
        let mut res = String::new();

        while let Some(next) = i.next() {
            if res.is_empty() {
                res.push_str(&format!("{next}"));
            } else {
                res.push_str(&format!("{sep}{next}"));
            }
        }

        res
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_format_one_condition() {
            assert_eq!(
                format_condition(&[Condition::EndsWith(EndsWith {
                    field: "from".to_owned(),
                    values: vec!["@example.com".to_owned()]
                })]),
                "AND (from,ends with,@example.com)"
            );
        }

        #[test]
        fn test_format_multiple_conditions() {
            assert_eq!(
                format_condition(&[Condition::EndsWith(EndsWith {
                    field: "from".to_owned(),
                    values: vec!["@example.com".to_owned(), "@test.com".to_owned()]
                })]),
                "OR (from,ends with,@example.com) OR (from,ends with,@test.com)"
            );
        }

        #[test]
        fn test_append_filter() {
            let mut doc = DatDocument::new();
            append_filter(
                &mut doc,
                "one".to_owned(),
                "test/one".to_owned(),
                &[Condition::EndsWith(EndsWith {
                    field: "subject".to_owned(),
                    values: vec!["test".to_owned()],
                })],
            );

            assert_eq!(
                doc.to_string(),
                vec![
                    "version=\"9\"",
                    "logging=\"no\"",
                    "name=\"one\"",
                    "enabled=\"yes\"",
                    "type=\"17\"",
                    "action=\"Move to folder\"",
                    "actionValue=\"test/one\"",
                    "condition=AND (subject,ends with,test)"
                ]
                .join("\n")
            );
        }

        #[test]
        fn test_iter_join() {
            assert_eq!(iter_join([1].iter(), ", "), "1");
            assert_eq!(iter_join([1, 2].iter(), ", "), "1, 2");
        }

        use super::*;
        use crate::configuration::EndsWith;
    }

    use crate::{configuration::Condition, dat::DatDocument};
}

use crate::{configuration::Configuration, dat::DatDocument};
