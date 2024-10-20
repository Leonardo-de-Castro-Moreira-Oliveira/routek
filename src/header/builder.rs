pub trait HttpHeaderBuilder {
    // Create a new header for http responses by key and value list.
    fn new(key: &str, values: Vec<&str>) -> Self
    where
        Self: Sized;

    // Create a new http header by key and empty list.
    fn empty(key: &str) -> Self
    where
        Self: Sized;
}
