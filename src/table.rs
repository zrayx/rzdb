use crate::data::Data;
use crate::row::Row;

pub struct Table {
    pub name: String,
    column_names: Vec<String>,
    rows: Vec<Row>,
}

impl Table {
    pub fn create(name: &str) -> Table {
        Table {
            name: name.to_string(),
            column_names: vec![],
            rows: vec![],
        }
    }

    pub fn create_column(&mut self, name: &str) {
        self.column_names.push(name.to_string());
    }

    pub fn insert(&mut self, values: Vec<&str>) {
        let mut row = Row::new();
        for value in &values {
            row.add_parse(value);
        }
        if self.column_names.len() != values.len() {
            panic!(
                "Table::insert({}, {:?}): tried to insert {} items, but have {} columns.",
                self.name,
                values,
                self.column_names.len(),
                values.len(),
            );
        }
        self.rows.push(row);
    }

    pub fn select(&self) -> Vec<Vec<Data>> {
        let mut result = vec![];
        for row in &self.rows {
            result.push(row.select());
        }
        result
    }
}
