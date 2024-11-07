pub struct XmlDocument {
    buff: String,
}

impl XmlDocument {
    pub fn new() -> Self {
        XmlDocument {
            buff: "<?xml version=\"1.0\"?>".to_owned(),
        }
    }

    pub fn append_element(&mut self, child: XmlElement) {
        helpers::xml_append(&mut self.buff, child.0);
    }

    pub fn to_string(self) -> String {
        self.buff
    }
}

pub struct XmlElement(String);

impl XmlElement {
    pub fn new(tag: String, attrs: String, content: String) -> Self {
        let xml = if !attrs.is_empty() {
            format!("<{tag} {attrs}>{content}</{tag}>")
        } else {
            format!("<{tag}>{content}</{tag}>")
        };
        XmlElement(xml)
    }
}

pub struct XmlElementBuilder {
    tag: String,
    attrs: Vec<(String, String)>,
    children: Vec<XmlElement>,
}

pub struct XmlTextElementBuilder {
    tag: String,
    attrs: Vec<(String, String)>,
    text: String,
}

impl XmlElementBuilder {
    pub fn new(tag: &str) -> Self {
        XmlElementBuilder {
            tag: tag.to_owned(),
            attrs: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn append_attr<S>(&mut self, name: &str, value: S) -> &mut Self
    where
        S: std::fmt::Display,
    {
        self.attrs.push((name.to_owned(), format!("\"{value}\"")));
        self
    }

    pub fn append_child(&mut self, child: XmlElement) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn build(self) -> XmlElement {
        let attrs = helpers::format_attrs(self.attrs);
        let children = helpers::format_children(self.children);
        XmlElement::new(self.tag, attrs, children)
    }
}

impl XmlTextElementBuilder {
    pub fn new(tag: &str) -> Self {
        XmlTextElementBuilder {
            tag: tag.to_owned(),
            attrs: Vec::new(),
            text: String::new(),
        }
    }

    pub fn append_attr<S>(&mut self, name: &str, value: S) -> &mut Self
    where
        S: std::fmt::Display,
    {
        self.attrs.push((name.to_owned(), format!("\"{value}\"")));
        self
    }

    pub fn append_text<S>(&mut self, text: S) -> &mut Self
    where
        S: std::fmt::Display,
    {
        self.text.push_str(&text.to_string());
        self
    }

    pub fn build(self) -> XmlElement {
        let attrs = helpers::format_attrs(self.attrs);
        XmlElement::new(self.tag, attrs, self.text)
    }
}

mod helpers {

    pub fn format_attrs(attrs: Vec<(String, String)>) -> String {
        attrs
            .into_iter()
            .map(|(name, value)| format!("{name}={value}"))
            .reduce(|lhs, rhs| format!("{lhs} {rhs}"))
            .unwrap_or_default()
    }

    pub fn format_children(children: Vec<XmlElement>) -> String {
        children
            .into_iter()
            .map(|child| child.0)
            .fold(String::new(), |acc, item| format!("{}{}", acc, item))
    }

    pub fn xml_append<S>(xml: &mut String, s: S)
    where
        S: std::fmt::Display,
    {
        xml.push_str(&s.to_string());
    }

    use super::XmlElement;
}
