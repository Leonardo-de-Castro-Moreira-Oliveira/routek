pub trait HttpHeaderBuilder {
    /// Takes a key (&str) and values (Vec<String>) and create/return
    /// a new Http Header for Responses and Requests.
    fn new(key: &str, values: Vec<String>) -> Self;

    /// Takes a key (&str) and values (Vec<&str>), parse the
    /// values to Vec<String> and create/return a new Http Header
    /// for Responses and Requests.
    fn build(key: &str, values: Vec<&str>) -> Self;

    /// Takes a key (&str), parse the key to string and create/return
    /// a new http header with no values.
    fn empty(key: &str) -> Self;
}
