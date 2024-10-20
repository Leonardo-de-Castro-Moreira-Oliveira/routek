use super::HttpHeader;

#[derive(Clone)]
pub struct HttpHeaders(pub Vec<HttpHeader>);

impl ToString for HttpHeaders {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|header| header.to_string())
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}
