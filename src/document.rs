use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    #[must_use]
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let mut rows = vec![];
        let contents = fs::read_to_string(filename)?;
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self { rows })
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
