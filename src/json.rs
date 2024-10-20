pub trait Json {
    fn serialize(&self) -> String;

    fn deserialize(json: String) -> Option<Self>
    where
        Self: Sized;
}
