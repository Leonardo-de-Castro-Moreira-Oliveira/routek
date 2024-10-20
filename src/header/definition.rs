use crate::header::HttpHeaderBuilder;

use super::HttpHeaderManager;

#[derive(Clone)]
pub struct HttpHeader {
    key: String,         // The key of http header.
    values: Vec<String>, // The values of http header.
}

impl HttpHeaderBuilder for HttpHeader {
    fn new(key: &str, values: Vec<String>) -> Self
    where
        Self: Sized,
    {
        HttpHeader {
            key: key.to_string(), // Parsing key to string.
            values,               // Defines the parsed values.
        }
    }

    fn build(key: &str, values: Vec<&str>) -> Self
    where
        Self: Sized,
    {
        HttpHeader {
            key: key.to_string(),                                   // Parsing key to string.
            values: values.iter().map(|v| v.to_string()).collect(), // Parsing the all values to string.
        }
    }

    fn empty(key: &str) -> Self {
        HttpHeader {
            key: key.to_string(),
            values: Vec::new(),
        }
    }
}

impl HttpHeaderManager for HttpHeader {
    fn set_key(&mut self, key: &str) -> &mut Self {
        self.key = key.to_string();
        self
    }

    fn set_values(&mut self, values: Vec<&str>) -> &mut Self {
        self.values = values.iter().map(|v| v.to_string()).collect();
        self
    }

    fn key_eq(&self, other: &str) -> bool {
        self.key.eq_ignore_ascii_case(other)
    }

    fn value_exists(&self, value: &str) -> bool {
        self.values.iter().any(|v| v.eq_ignore_ascii_case(value))
    }

    fn insert_value(&mut self, value: &str) -> &mut Self {
        if self.value_exists(value) {
            let new = value.to_string();
            self.values.push(new);
        }
        self
    }

    fn remove_value(&mut self, value: &str) -> &mut Self {
        self.values.retain(|v| !v.eq_ignore_ascii_case(value));
        self
    }
}

impl ToString for HttpHeader {
    fn to_string(&self) -> String {
        let values = self.values.join(", "); // Parsing the list to string by ', '
        format!("{}: {}", self.key, values) // Return the response format, ex: Content-Type: text/json
    }
}
