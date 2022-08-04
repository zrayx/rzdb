use crate::data::Data;
use std::error::Error;

pub struct Row {
    data: Vec<Data>,
}

impl Clone for Row {
    fn clone(&self) -> Row {
        Row {
            data: self.data.clone(),
        }
    }
}

impl Row {
    pub fn new() -> Row {
        Row { data: vec![] }
    }

    pub fn from(data: Vec<Data>) -> Row {
        Row { data }
    }

    pub fn add(&mut self, data: Data) {
        self.data.push(data);
    }

    pub fn add_parse(&mut self, s: &str) {
        let data = Data::parse(s);
        self.add(data);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn select(&self) -> Vec<Data> {
        let mut result = vec![];
        for datum in &self.data {
            result.push(datum.clone());
        }
        result
    }

    pub fn select_at_multiple(&self, indices: &[usize]) -> Row {
        let mut result = vec![];
        for index in indices {
            result.push(self.data[*index].clone());
        }
        Row::from(result)
    }

    pub fn select_at(&self, idx: usize) -> Result<Data, Box<dyn Error>> {
        if idx >= self.data.len() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Row::select_at({}): index out of bounds ({} columns)",
                    idx,
                    self.data.len(),
                ),
            )));
        }
        Ok(self.data[idx].clone())
    }

    pub fn set_at(&mut self, idx: usize, value: Data) -> Result<(), Box<dyn Error>> {
        self.data[idx] = value;
        Ok(())
    }

    pub fn insert_at(&mut self, idx: usize, value: Data) {
        self.data.insert(idx, value);
    }
    pub fn insert_columns_at(&mut self, index: usize, other: &Row) {
        self.data.splice(index..index, other.data.iter().cloned());
    }
    pub fn delete(&mut self, idx: usize) {
        self.data.remove(idx);
    }

    pub fn iter(&self) -> std::slice::Iter<Data> {
        self.data.iter()
    }
}
impl std::iter::IntoIterator for Row {
    type Item = Data;
    type IntoIter = std::vec::IntoIter<Data>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
impl Default for Row {
    fn default() -> Row {
        Row::new()
    }
}
