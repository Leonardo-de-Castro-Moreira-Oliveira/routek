pub trait Json {
    /// Parse this datatype to json on String.
    fn serialize(&self) -> String;

    /// Takes a buffer (String) and parse to Self type
    /// returning the Option::Some(Self) in case of success.
    fn deserialize(buffer: String) -> Option<Self>
    where
        Self: Sized;
}
