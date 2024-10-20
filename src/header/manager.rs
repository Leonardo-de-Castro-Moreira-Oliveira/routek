pub trait HttpHeaderManager {
    /// Takes a string and set the key of http header.
    fn set_key(&mut self, key: &str) -> &mut Self;

    /// Takes a list of &str and set the values of http header.
    fn set_values(&mut self, values: Vec<&str>) -> &mut Self;

    /// Check if the key is equal other.
    fn key_eq(&self, other: &str) -> bool;

    /// Check if the value exists in the header;
    fn value_exists(&self, value: &str) -> bool;

    /// Takes a &str and try to add one value to header.
    fn insert_value(&mut self, value: &str) -> &mut Self;

    /// Takes a &str and try to remove one value of header.
    fn remove_value(&mut self, value: &str) -> &mut Self;
}
