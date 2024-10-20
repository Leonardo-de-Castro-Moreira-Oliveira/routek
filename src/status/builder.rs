pub trait HttpStatusBuilder {
    /// Takes a version(String) and value(u16), and create/return
    /// a new http status with the defined by status.
    fn new(version: String, value: u16) -> Option<Self>
    where
        Self: Sized;

    /// Takes a version(&str) and value(u16), parse version
    /// to string and create/return a new http status with the sign
    /// defined by status.
    fn build(version: &str, value: u16) -> Option<Self>
    where
        Self: Sized;

    /// Takes a version(&str), parse version to string and create/return
    /// a new http status with empty value and sign.
    fn empty(version: &str) -> Self;
}
