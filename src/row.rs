use crate::data::Data;

pub struct Row {
    data: Vec<Data>,
}

impl Row {
    pub fn new() -> Row {
        Row { data: vec![] }
    }

    pub fn add_parse(&mut self, s: &str) {
        let data = Data::parse(s);
        self.data.push(data);
    }

    pub fn select(&self) -> Vec<Data> {
        let mut result = vec![];
        for datum in &self.data {
            result.push(datum.clone());
        }
        result
    }
}
