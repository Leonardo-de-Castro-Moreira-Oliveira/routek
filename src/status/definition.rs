use super::HttpStatusBuilder;

#[derive(Clone)]
pub struct HttpStatus {
    version: String,
    value: u16,
    sign: String,
}

impl HttpStatusBuilder for HttpStatus {
    fn empty(version: &str) -> Self {
        HttpStatus {
            version: version.to_string(),
            value: 0,
            sign: String::new(),
        }
    }
}

impl ToString for HttpStatus {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.version, self.value, self.sign)
    }
}
