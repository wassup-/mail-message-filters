pub struct XmlDocument {
    buff: String,
}

pub struct XmlElement {
    tag: &'static str,
    buff: String,
}

impl XmlDocument {
    pub fn new() -> Self {
        XmlDocument {
            buff: "<?xml version=\"1.0\"?>".to_owned(),
        }
    }

    pub fn append_element(&mut self, child: XmlElement) {
        xml_append(&mut self.buff, child.close());
    }

    pub fn finish(self) -> String {
        self.buff
    }
}

impl XmlElement {
    pub fn open(tag: &'static str) -> XmlElement {
        XmlElement {
            tag,
            buff: format!("<{}>", tag),
        }
    }

    pub fn open_attr<S>(tag: &'static str, attr: S) -> XmlElement
    where
        S: std::fmt::Display,
    {
        XmlElement {
            tag,
            buff: format!("<{} {}>", tag, attr),
        }
    }

    pub fn close(mut self) -> String {
        xml_append(&mut self.buff, format!("</{}>", self.tag));
        self.buff
    }

    pub fn append_child(&mut self, child: XmlElement) {
        xml_append(&mut self.buff, child.close());
    }

    pub fn append<S>(&mut self, s: S)
    where
        S: std::fmt::Display,
    {
        xml_append(&mut self.buff, s);
    }
}

fn xml_append<S>(xml: &mut String, s: S)
where
    S: std::fmt::Display,
{
    xml.push_str(&s.to_string());
}
