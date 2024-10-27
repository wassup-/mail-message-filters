pub struct DatDocument {
    lines: Vec<String>,
}

impl DatDocument {
    pub fn new(version: u16, logging: bool) -> Self {
        let mut doc = DatDocument { lines: Vec::new() };
        doc.append("version", quote(version));
        doc.append("logging", quote(if logging { "yes" } else { "no" }));
        doc
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }

    pub fn append<S>(&mut self, name: &str, value: S)
    where
        S: std::fmt::Display,
    {
        self.lines.push(format!("{name}={value}"));
    }
}

use crate::util::quote;
