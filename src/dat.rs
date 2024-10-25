pub struct DatDocument {
    lines: Vec<String>,
}

impl DatDocument {
    pub fn new() -> Self {
        let mut doc = DatDocument { lines: Vec::new() };
        doc.append_str("version", "9");
        doc.append_str("logging", "no");
        doc
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }

    pub fn append_str<S>(&mut self, name: &str, value: S)
    where
        S: std::fmt::Display,
    {
        self.lines.push(format!("{name}=\"{value}\""));
    }

    pub fn append<S>(&mut self, name: &str, value: S)
    where
        S: std::fmt::Display,
    {
        self.lines.push(format!("{name}={value}"));
    }
}
