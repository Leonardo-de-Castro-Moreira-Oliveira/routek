pub trait HttpStatusManager {
    /// Takes a version (&str) and set the version
    /// of this http status to the specified version.
    fn set_version(&mut self, version: &str) -> &mut Self;

    /// Takes a value (u16) and set the value of
    /// this http status to the specified value and
    /// set the sign by values.
    fn set_value(&mut self, value: u16) -> Option<&mut Self>;

    /// Takes a value (u16) and return the http status
    /// sign with this value.
    fn get_sign(value: u16) -> Option<String>;
}
