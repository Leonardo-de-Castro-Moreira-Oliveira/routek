pub trait HttpServerManager {
    // Add one value to this header.
    fn insert(&mut self, value: &str);

    // Remove one value of this header.
    fn remove(&mut self, value: &str);

    // Check if the key is equl to other.
    fn key_eq(&self, key: &str) -> bool;
}
