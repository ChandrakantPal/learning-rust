struct Uppercase(String);

impl From<String> for Uppercase {
    fn from(data: String) -> Self {
        Uppercase(data.to_uppercase())
    }
}
