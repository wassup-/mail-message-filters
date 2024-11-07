pub struct DatDocument {
    lines: Vec<String>,
}

impl DatDocument {
    pub fn new(version: u16, logging: bool) -> Self {
        let mut doc = DatDocument { lines: Vec::new() };
        doc.append("version", version);
        doc.append("logging", if logging { "yes" } else { "no" });
        doc
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }

    pub fn append<S>(&mut self, name: &str, value: S)
    where
        S: std::fmt::Display,
    {
        self.lines.push(format!("{name}=\"{value}\""));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn append_inserts_quotes() {
        let mut doc = DatDocument { lines: Vec::new() };
        doc.append("foo", "bar");
        assert_eq!(doc.lines[0], "foo=\"bar\"");
    }

    use super::*;
}
