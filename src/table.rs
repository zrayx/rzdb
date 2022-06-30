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

    pub fn get_column_names(&self) -> Vec<String> {
        self.column_names.clone()
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn column_count(&self) -> usize {
        self.column_names.len()
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let pad = |s: &str, width: usize| {
            let mut s = s.to_string();
            while s.len() < width {
                s.push(' ');
            }
            s
        };
        let line = |width| {
            let mut s = String::new();
            for _ in 0..width {
                s.push('-');
            }
            s
        };

        // get the width of all column names
        let mut column_widths = vec![];
        for column_name in &self.column_names {
            column_widths.push(column_name.len());
        }
        // get the maximum width of all row values
        for row in &self.rows {
            for (i, value) in row.select().iter().enumerate() {
                let width = column_widths[i];
                let value = value.to_string();
                if value.len() > width {
                    column_widths[i] = value.len();
                }
            }
        }

        let mut result = String::new();
        // write column names
        for (i, column_name) in self.column_names.iter().enumerate() {
            let width = column_widths[i];
            result.push_str(&pad(column_name, width + 1));
        }
        result.push('\n');
        // write line under column names
        for (i, _) in self.column_names.iter().enumerate() {
            let width = column_widths[i];
            result.push_str(&line(width));
            result.push(' ');
        }
        result.push('\n');

        // write row values
        for row in &self.rows {
            for (i, value) in row.select().iter().enumerate() {
                let width = column_widths[i];
                result.push_str(&pad(&value.to_string(), width + 1));
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}
