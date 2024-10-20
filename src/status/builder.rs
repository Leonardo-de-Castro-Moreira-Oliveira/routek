pub trait HttpStatusBuilder {
    // Create a empty status for http response with version.
    fn empty(version: &str) -> Self;
}
