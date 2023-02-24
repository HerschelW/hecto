pub struct Row {
    string: String,
}

impl Row {
    pub fn from(string: &str) -> Self {
        Self {
            string: String::from(string),
        }
    }
}
