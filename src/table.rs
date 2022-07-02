use crate::data::Data;
use crate::row::Row;
use std::error::Error;

pub struct Table {
    pub name: String,
    column_names: Vec<String>,
    rows: Vec<Row>,
    changed: bool,
}

impl Table {
    pub fn create(name: &str) -> Table {
        Table {
            name: name.to_string(),
            column_names: vec![],
            rows: vec![],
            changed: false,
        }
    }

    pub fn load(full_name: &str) -> Result<Table, Box<dyn Error>> {
        let content = std::fs::read_to_string(full_name)?;

        let mut lines = content.lines();
        let column_names: Vec<String> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        // rows
        let column_count = column_names.len();
        let mut rows = vec![];
        for line in lines {
            let mut row = Row::new();
            let mut data = Data::decode_line(line);
            while data.len() < column_count {
                data.push(Data::Empty);
            }
            for datum in data {
                row.add(datum);
            }
            rows.push(row);
        }

        // table name
        let name = full_name
            .split('/')
            .last()
            .unwrap()
            .split('.')
            .next()
            .unwrap()
            .to_string();

        Ok(Table {
            name,
            column_names,
            rows,
            changed: false,
        })
    }

    pub fn save(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut out = String::new();
        for (idx, name) in self.get_column_names().iter().enumerate() {
            if idx > 0 {
                out.push(',');
            }
            out.push_str(name);
        }
        out.push('\n');

        let all_data = self.select();
        for row in all_data {
            for (idx, value) in row.iter().enumerate() {
                if idx > 0 {
                    out.push(',');
                }
                out.push_str(&value.encode_for_csv());
            }
            out.push('\n');
        }

        std::fs::write(filename, out)?;
        self.changed = false;
        Ok(())
    }

    pub fn is_changed(&self) -> bool {
        self.changed
    }

    pub fn get_column_idx(&self, name: &str) -> Option<usize> {
        self.column_names.iter().position(|n| n == name)
    }

    pub fn create_column(&mut self, name: &str) {
        self.column_names.push(name.to_string());
        for row in &mut self.rows {
            row.add(Data::Empty);
        }
        self.changed = true;
    }

    pub fn insert_row_at(&mut self, index: usize) {
        let column_count = self.column_count();
        self.rows
            .insert(index, Row::new_from(vec![Data::Empty; column_count]));
        self.changed = true;
    }

    pub fn insert_column_at(&mut self, column_name: &str, idx: usize) {
        self.column_names.insert(idx, column_name.to_string());
        for row in &mut self.rows {
            row.insert_at(idx, Data::Empty);
        }
        self.changed = true;
    }

    pub fn delete_row(&mut self, row_idx: usize) {
        self.rows.remove(row_idx);
        self.changed = true;
    }

    pub fn delete_column(&mut self, column_name: &str) {
        let idx = self.get_column_idx(column_name);
        if idx == None {
            panic!(
                "Table::delete_column: could not find column {}",
                column_name
            );
        }
        let idx = idx.unwrap();
        for row in &mut self.rows {
            row.delete(idx);
        }
        self.column_names.remove(idx);
        self.changed = true;
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
        self.changed = true;
    }

    pub fn select(&self) -> Vec<Vec<Data>> {
        let mut result = vec![];
        for row in &self.rows {
            result.push(row.select());
        }
        result
    }

    pub fn select_at(&self, col_idx: usize, row_idx: usize) -> Data {
        self.rows[row_idx].select_at(col_idx)
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

    pub fn set_at(
        &mut self,
        row_idx: usize,
        column_idx: usize,
        value: Data,
    ) -> Result<(), Box<dyn Error>> {
        self.changed = true;
        self.rows[row_idx].set_at(column_idx, value)
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
