use super::{HttpHeaderBuilder, HttpServerManager};

#[derive(Clone)]
pub struct HttpHeader {
    key: String,
    values: Vec<String>,
}

impl HttpHeaderBuilder for HttpHeader {
    fn new(key: &str, values: Vec<&str>) -> Self {
        HttpHeader {
            key: key.to_string(),
            values: values.iter().map(|v| v.to_string()).collect(),
        }
    }

    fn empty(key: &str) -> Self
    where
        Self: Sized,
    {
        HttpHeader {
            key: key.to_string(),
            values: Vec::new(),
        }
    }
}

impl HttpServerManager for HttpHeader {
    fn insert(&mut self, value: &str) {
        if let None = self.values.iter().find(|v| v.eq_ignore_ascii_case(value)) {
            let parsed = value.to_string();
            self.values.push(parsed);
        }
    }

    fn remove(&mut self, value: &str) {
        self.values.retain(|v| v.eq_ignore_ascii_case(value));
    }

    fn key_eq(&self, key: &str) -> bool {
        self.key.eq_ignore_ascii_case(key)
    }
}

impl ToString for HttpHeader {
    fn to_string(&self) -> String {
        let concatenated = self.values.join(", ");
        format!("{}: {}", self.key, concatenated)
    }
}
