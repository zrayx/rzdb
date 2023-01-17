use crate::condition::Condition;
use crate::data::Data;
use crate::row::Row;
use std::error::Error;

pub struct Table {
    name: String,
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

        let lines = content.lines();
        let mut column_names: Vec<String> = vec![];

        // rows
        let mut rows = vec![];
        for (idx, line) in lines.enumerate() {
            let mut row = Row::new();
            let mut data = Data::decode_line(line);
            if idx == 0 {
                for datum in data {
                    column_names.push(datum.to_string());
                }
            } else {
                let num_columns = column_names.len();
                if data.len() > num_columns {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!(
                            "Table::load({}): table has {} columns, but row nr. {} has {} columns)",
                            full_name,
                            data.len(),
                            idx,
                            num_columns,
                        ),
                    )));
                }
                while data.len() < num_columns {
                    data.push(Data::Empty);
                }
                for datum in data {
                    row.add(datum);
                }
                rows.push(row);
            }
        }

        // table name
        let name = full_name
            .split('/')
            .last()
            .unwrap()
            .split(".csv")
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
            let encoded = Data::String(name.clone()).encode_for_csv();
            out.push_str(&encoded.to_string());
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

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_changed(&self) -> bool {
        self.changed
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_column_idx_result(&self, name: &str) -> Result<usize, Box<dyn Error>> {
        if let Some(idx) = self.column_names.iter().position(|n| n == name) {
            Ok(idx)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Column {} not found", name),
            )))
        }
    }

    pub fn get_column_idx_option(&self, name: &str) -> Option<usize> {
        self.column_names.iter().position(|n| n == name)
    }

    pub fn create_column(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        if self.get_column_idx_option(name).is_some() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("column {} already exists", name),
            )));
        }
        self.column_names.push(name.to_string());
        for row in &mut self.rows {
            row.add(Data::Empty);
        }
        self.changed = true;
        Ok(())
    }

    pub fn rename_column(&mut self, old_name: &str, new_name: &str) -> Result<(), Box<dyn Error>> {
        if self.get_column_idx_result(new_name).is_ok() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("column {} already exists", new_name),
            )));
        }
        let idx = self.get_column_idx_result(old_name)?;
        self.column_names[idx] = new_name.to_string();
        self.changed = true;
        Ok(())
    }

    pub fn insert_empty_row_at(&mut self, index: usize) {
        let column_count = self.column_count();
        self.rows
            .insert(index, Row::from(vec![Data::Empty; column_count]));
        self.changed = true;
    }
    pub fn insert_rows_at(&mut self, index: usize, rows: Vec<Row>) {
        self.rows.splice(index..index, rows);
        self.changed = true;
    }
    pub fn insert_column_at(&mut self, column_name: &str, idx: usize) {
        self.column_names.insert(idx, column_name.to_string());
        for row in &mut self.rows {
            row.insert_at(idx, Data::Empty);
        }
        self.changed = true;
    }

    pub fn delete_all(&mut self) {
        self.column_names.clear();
        self.rows.clear();
        self.changed = true;
    }

    pub fn delete_row(&mut self, row_idx: usize) {
        self.rows.remove(row_idx);
        self.changed = true;
    }

    pub fn delete_column(&mut self, column_name: &str) -> Result<(), Box<dyn Error>> {
        if let Some(idx) = self.get_column_idx_option(column_name) {
            for row in &mut self.rows {
                row.delete(idx);
            }
            self.column_names.remove(idx);
            self.changed = true;
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("column {} not found", column_name),
            )))
        }
    }

    pub fn insert(&mut self, values: Vec<&str>) -> Result<(), Box<dyn Error>> {
        let mut row = Row::new();
        for value in &values {
            row.add_parse(value);
        }
        if self.column_names.len() != values.len() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Table::insert({}, {:?}): tried to insert {} items, but have {} columns.",
                    self.name,
                    values,
                    self.column_names.len(),
                    values.len(),
                ),
            )));
        }
        self.rows.push(row);
        self.changed = true;
        Ok(())
    }

    pub fn insert_data(&mut self, data: Vec<Data>) -> Result<(), Box<dyn Error>> {
        if self.column_names.len() != data.len() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Table::insert({}, {:?}): tried to insert {} items, but have {} columns.",
                    self.name,
                    data,
                    self.column_names.len(),
                    data.len(),
                ),
            )));
        }
        self.rows.push(Row::from(data));
        self.changed = true;
        Ok(())
    }
    pub fn insert_into_at(&mut self, index: usize, rows: Vec<Row>) {
        self.rows.splice(index..index, rows);
        self.changed = true;
    }
    pub fn insert_columns_at(&mut self, index: usize, table: &Table) -> Result<(), Box<dyn Error>> {
        // check number of rows is equal
        if self.row_count() != table.row_count() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Table::insert_columns_at({}, {:?}): tried to insert {} rows, but have {} rows.",
                    self.name,
                    table.name,
                    self.row_count(),
                    table.row_count(),
                ),
            )));
        }
        // check for duplicates; TODO: use any(), because it looks cooler
        let self_names = self.get_column_names();
        let other_names = table.get_column_names();
        for name in other_names {
            if self_names.contains(&name) {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "Table::insert_columns_at({}, {}): column {} already exists",
                        self.name, table.name, name
                    ),
                )));
            }
        }
        // insert column headers
        self.column_names
            .splice(index..index, table.get_column_names());
        // insert columns
        for (row_index, row) in &mut self.rows.iter_mut().enumerate() {
            row.insert_columns_at(index, &table.rows[row_index].clone());
        }
        self.changed = true;
        Ok(())
    }

    pub fn append_rows(&mut self, rows: &mut Vec<Row>) -> Result<(), Box<dyn Error>> {
        if !rows.is_empty() && self.column_count() != rows.first().unwrap().len() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Table::append_rows() (table: {}): tried to append {} items, but have {} columns.",
                    self.name,
                    self.column_names.len(),
                    rows.first().unwrap().len(),
                ),
            )));
        }
        self.rows.append(rows);
        self.changed = true;
        Ok(())
    }

    pub fn select(&self) -> Vec<Row> {
        let mut result = vec![];
        for row in &self.rows {
            result.push(row.clone());
        }
        result
    }

    pub fn select_columns(&self, column_names: &[&str]) -> Result<Vec<Row>, Box<dyn Error>> {
        let mut column_ids = vec![];
        for column_name in column_names {
            if let Some(idx) = self.get_column_idx_option(column_name) {
                column_ids.push(idx);
            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "Table::select_columns({}): column {} not found.",
                        self.name, column_name,
                    ),
                )));
            }
        }

        let mut result = vec![];
        for row in &self.rows {
            let mut new_row = Row::new();
            for column_id in &column_ids {
                new_row.add(row.select_at(*column_id).unwrap());
            }
            result.push(new_row);
        }
        Ok(result)
    }

    pub fn select_at(&self, col_idx: usize, row_idx: usize) -> Result<Data, Box<dyn Error>> {
        if row_idx >= self.rows.len() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Table::select_at({}, {}): row index out of bounds ({} rows)",
                    col_idx,
                    row_idx,
                    self.rows.len()
                ),
            )));
        }
        let len = self.rows[row_idx].len();
        if col_idx >= len {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Table::select_at({}, {}): column index out of bounds ({} columns)",
                    col_idx, row_idx, len
                ),
            )));
        }
        self.rows[row_idx].select_at(col_idx)
    }

    pub fn select_where(&self, conditions: &[Condition]) -> Result<Vec<Row>, Box<dyn Error>> {
        // find the ids that match the columns
        let mut column_ids = vec![];
        let column_names = self.get_column_names();
        for condition in conditions {
            if let Some(idx) = column_names.iter().position(|x| x == condition.column) {
                column_ids.push(idx);
            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "Table::select_where({}): column {} not found.",
                        self.name, condition.column,
                    ),
                )));
            }
        }

        // find the rows that match the conditions
        let mut result = vec![];
        for row in self.select() {
            let mut matches = true;
            for (condition_index, condition) in conditions.iter().enumerate() {
                let column_id = column_ids[condition_index];
                let value = row.select_at(column_id).unwrap();
                if !condition.matches(&value) {
                    matches = false;
                    break;
                }
            }
            if matches {
                result.push(row.clone());
            }
        }
        Ok(result)
    }

    pub fn get_column_name_at(&self, idx: usize) -> String {
        self.column_names[idx].clone()
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
