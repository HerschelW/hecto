pub struct Row {
    string: String,
}

impl Row {
    #[must_use]
    pub fn from(string: &str) -> Self {
        Self {
            string: String::from(string),
        }
    }
}
