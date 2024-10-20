use crate::{
    header::HttpHeaders,
    json::Json,
    status::{HttpStatus, HttpStatusBuilder},
};

use super::HttpResponseBuilder;

#[derive(Clone)]
pub struct HttpResponse<T>
where
    T: Json,
{
    pub status: HttpStatus,
    pub headers: HttpHeaders,
    pub body: T,
}

impl<T> HttpResponseBuilder<T> for HttpResponse<T>
where
    T: Json,
{
    fn new(status: HttpStatus, headers: HttpHeaders, body: T) -> Self
    where
        T: Json,
    {
        HttpResponse {
            status,
            headers,
            body,
        }
    }

    fn from_body(version: &str, body: T) -> Self
    where
        T: Json,
    {
        HttpResponse {
            status: HttpStatus::empty(version),
            headers: HttpHeaders { 0: Vec::new() },
            body,
        }
    }
}

impl<T> ToString for HttpResponse<T>
where
    T: Json,
{
    fn to_string(&self) -> String {
        let status = self.status.to_string();
        let headers = if self.headers.0.is_empty() {
            "Content-Length: 0".to_string() // Pode definir um header padrão se estiver vazio
        } else {
            self.headers.to_string()
        };
        let body = self.body.serialize();
        format!("{}\r\n{}\r\n\r\n{}", status, headers, body)
    }
}
