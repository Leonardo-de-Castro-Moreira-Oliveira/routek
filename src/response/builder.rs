use crate::{header::HttpHeaders, json::Json, status::HttpStatus};

pub trait HttpResponseBuilder<T>
where
    T: Json,
{
    /// Takes a status, headers and Json body, create/return a new
    /// http response with all properties.
    fn new(status: HttpStatus, headers: HttpHeaders, body: T) -> Self;

    /// Takes a version and Json body, create/return a new http response
    /// with status version and body.
    fn from_body(version: &str, body: T) -> Self;
}
