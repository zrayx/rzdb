use std::error::Error;

use crate::condition::Condition;
use crate::data::Data;
use crate::join::Join;
use crate::row::Row;
use crate::table::Table;
use crate::time::Timestamp;

pub struct Db {
    pub name: String,
    pub db_dir: String,
    tables: Vec<Table>,
}

const IDS_TABLE_ID: usize = 0;
const IDS_COLUMN_ID: usize = 2;

impl Db {
    fn expand_home_dir(path: &str) -> String {
        if !path.is_empty() {
            if path.starts_with('~') {
                let user_home_dir = std::env::var("HOME").unwrap();
                format!(
                    "{}{}",
                    user_home_dir,
                    path.to_string().chars().skip(1).collect::<String>(),
                )
            } else {
                path.to_string()
            }
        } else {
            "".to_string()
        }
    }
    fn path_names(&self) -> (String, String) {
        let expanded_db_dir = format!("{}/{}", Db::expand_home_dir(&self.db_dir), self.name);
        let backup_path = format!("{}/backup", &expanded_db_dir);
        (expanded_db_dir, backup_path)
    }
    pub fn get_db_path(&self) -> String {
        Db::expand_home_dir(&self.db_dir)
    }

    fn new(name: &str, db_dir: &str) -> Db {
        let mut db = Db {
            name: name.to_string(),
            db_dir: db_dir.to_string(),
            tables: vec![],
        };
        db.create_table(".ids").unwrap();
        db.create_column(".ids", "id").unwrap();
        db.create_column(".ids", "references").unwrap();
        db.create_column(".ids", "content").unwrap();
        db
    }
    pub fn create(name: &str, db_dir: &str) -> Result<Db, Box<dyn Error>> {
        Ok(Db::new(name, db_dir))
    }

    fn table_filename(&self, table_name: &str) -> String {
        let (full_path, _) = self.path_names();
        format!("{}/{}.csv", full_path, table_name)
    }

    pub fn load(name: &str, db_dir: &str) -> Result<Db, Box<dyn Error>> {
        let mut db = Db {
            name: name.to_string(),
            db_dir: db_dir.to_string(),
            tables: vec![],
        };
        let (full_path, _) = db.path_names();
        let ids_file_name = format!("{}/.ids.csv", &full_path);
        if !std::path::Path::new(&ids_file_name).exists() {
            db.create_table(".ids").unwrap();
            db.create_column(".ids", "id").unwrap();
            db.create_column(".ids", "references").unwrap();
            db.create_column(".ids", "content").unwrap();
        } else {
            let ids_table = Table::load(&ids_file_name)?;
            db.tables.push(ids_table);
        }

        // load all other tables
        for entry in std::fs::read_dir(&full_path)? {
            let entry = entry?;
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename != ".ids" {
                let full_filename = format!("{}/{}", &full_path, filename);
                if filename.ends_with(".csv") {
                    let table = Table::load(&full_filename)?;
                    db.tables.push(table);
                }
            }
        }
        Ok(db)
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        let (full_path, backup_path) = self.path_names();
        std::fs::create_dir_all(&full_path).unwrap();
        std::fs::create_dir_all(&backup_path).unwrap();

        for table in &mut self.tables {
            if table.get_name() != "." && table.is_changed() {
                let filename = format!("{}/{}.csv", &full_path, table.get_name());

                let timestamp = Timestamp::now().to_filename_string();
                let backup_filename =
                    format!("{}/{}-{}.csv", &backup_path, table.get_name(), timestamp);

                if std::fs::metadata(&filename).is_ok() {
                    std::fs::copy(&filename, &backup_filename)?;
                }

                table.save(&filename)?;
            }
        }

        Ok(())
    }
    pub fn get_database_names(&self) -> Result<Vec<String>, Box<dyn Error>> {
        // list directories in db_dir
        let mut db_names = Vec::new();
        let db_dir = Db::expand_home_dir(&self.db_dir);
        for entry in std::fs::read_dir(&db_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                db_names.push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }
        Ok(db_names)
    }
    pub fn get_table_names(&self) -> Vec<String> {
        self.tables.iter().map(|t| t.get_name()).collect()
    }
    pub fn to_string(&self, table_name: &str) -> Result<String, Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        Ok(self.tables[id].to_string())
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn create_table(&mut self, table_name: &str) -> Result<(), Box<dyn Error>> {
        for table in &mut self.tables {
            if table.get_name() == table_name {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "Table already exists",
                )));
            }
        }
        self.tables.push(Table::create(table_name));
        Ok(())
    }

    pub fn create_or_replace_table(&mut self, table_name: &str) -> Result<(), Box<dyn Error>> {
        for table in &mut self.tables {
            if table.get_name() == table_name {
                table.delete_all();
                return Ok(());
            }
        }
        self.tables.push(Table::create(table_name));
        Ok(())
    }

    /// saves the table to backup, removes the table from the database in memory and deletes the file
    pub fn drop_table(&mut self, table_name: &str) -> Result<(), Box<dyn Error>> {
        // save database, ignore error if it fails
        if self.save().is_err() {}

        for (idx, table) in self.tables.iter().enumerate() {
            if table.get_name() == table_name {
                self.tables.remove(idx);
                break;
            }
        }

        // remove filename
        let filename = self.table_filename(table_name);
        std::fs::remove_file(&filename)?;
        Ok(())
    }

    pub fn create_column(
        &mut self,
        table_name: &str,
        column_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].create_column(column_name)
    }

    pub fn rename_column(
        &mut self,
        table_name: &str,
        old_name: &str,
        new_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].rename_column(old_name, new_name)
    }

    pub fn insert(&mut self, table_name: &str, values: Vec<&str>) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].insert(values)
    }

    /// insert a row at the specified index
    /// ```
    /// use rzdb::{Db,Data};
    /// let mut db = Db::create("test", "~/.local/rzdb").unwrap();
    /// let table_name = "insert_at";
    /// db.create_or_replace_table(table_name).unwrap();
    /// db.create_column(table_name, "col1").unwrap();
    /// db.create_column(table_name, "col2").unwrap();
    /// db.create_column(table_name, "col3").unwrap();
    /// db.insert(table_name, vec!["1", "2", "3"]).unwrap();
    /// db.insert_at(table_name, vec!["4", "5", "6"], 0).unwrap();
    /// db.insert_at(table_name, vec!["7", "8", "9"], 1).unwrap();
    /// let r = db.select_from(table_name).unwrap();
    /// assert_eq!(r[0].select_at(0).unwrap(), Data::Int(4));
    /// assert_eq!(r[1].select_at(0).unwrap(), Data::Int(7));
    /// assert_eq!(r[2].select_at(0).unwrap(), Data::Int(1));
    /// ```
    pub fn insert_at(
        &mut self,
        table_name: &str,
        values: Vec<&str>,
        idx: usize,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].insert_at(values, idx)
    }

    pub fn insert_data(
        &mut self,
        table_name: &str,
        values: Vec<Data>,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].insert_data(values)
    }

    pub fn insert_column_at(
        &mut self,
        table_name: &str,
        column_name: &str,
        index: usize,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].insert_column_at(column_name, index);
        Ok(())
    }

    pub fn insert_empty_row_at(
        &mut self,
        table_name: &str,
        index: usize,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].insert_empty_row_at(index);
        Ok(())
    }
    pub fn insert_rows_at(
        &mut self,
        table_name: &str,
        index: usize,
        rows: Vec<Row>,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].insert_rows_at(index, rows);
        Ok(())
    }
    /// insert all of source_table's rows into dest_table at index
    pub fn insert_into_at(
        &mut self,
        source_table: &str,
        dest_table: &str,
        row_idx: usize,
    ) -> Result<(), Box<dyn Error>> {
        let source_id = self.get_table_id(source_table)?;
        let dest_id = self.get_table_id(dest_table)?;
        let source_table = &self.tables[source_id];
        let rows = source_table.select();
        self.tables[dest_id].insert_into_at(row_idx, rows);
        Ok(())
    }
    /// insert all of source_table's columns into dest_table at index
    /// all columns must be unique, no duplicates allowed
    pub fn insert_columns_at(
        &mut self,
        source_table: &str,
        dest_table: &str,
        col_idx: usize,
    ) -> Result<(), Box<dyn Error>> {
        let source_id = self.get_table_id(source_table)?;
        let dest_id = self.get_table_id(dest_table)?;
        let (source_table, dest_table) = if source_id < dest_id {
            let tables = self.tables.split_at_mut(dest_id);
            (&tables.0[source_id], &mut tables.1[0])
        } else {
            let tables = self.tables.split_at_mut(source_id);
            (&tables.1[dest_id], &mut tables.0[0])
        };
        dest_table.insert_columns_at(col_idx, source_table)
    }

    /// insert values into table, or update if search condition met
    /// ```
    /// use rzdb::{Db,Condition,Data};
    /// let mut db = Db::create("test", "~/.local/rzdb").unwrap();
    /// let table_name = "insert_or_update";
    /// db.create_or_replace_table(table_name).unwrap();
    /// db.create_column(table_name, "col1").unwrap();
    /// db.create_column(table_name, "col2").unwrap();
    /// db.create_column(table_name, "col3").unwrap();
    /// db.insert(table_name, vec!["1", "2", "3"]).unwrap();
    /// db.insert(table_name, vec!["4", "5", "6"]).unwrap();
    /// db.insert(table_name, vec!["7", "8", "9"]).unwrap();
    /// let cond = Condition::equal_int("col1", 4);
    /// db.insert_update_where(table_name, vec!["10", "11", "12"], &[cond]).unwrap();
    /// let r = db.select_from(table_name).unwrap();
    /// assert_eq!(r[1].select_at(0).unwrap(), Data::Int(10));
    /// assert_eq!(r[1].select_at(1).unwrap(), Data::Int(11));
    /// assert_eq!(r[1].select_at(2).unwrap(), Data::Int(12));
    /// let cond = Condition::equal_int("col1", 4);
    /// db.insert_update_where(table_name, vec!["4", "5", "6"], &[cond]).unwrap();
    /// let r = db.select_from(table_name).unwrap();
    /// assert_eq!(r[3].select_at(0).unwrap(), Data::Int(4));
    /// assert_eq!(r[3].select_at(1).unwrap(), Data::Int(5));
    /// assert_eq!(r[3].select_at(2).unwrap(), Data::Int(6));
    /// ```
    pub fn insert_update_where(
        &mut self,
        table_name: &str,
        values: Vec<&str>,
        conditions: &[Condition],
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].insert_update_where(values, conditions)
    }

    /// delete all rows that match the conditions
    /// ```
    /// use rzdb::{Db,Condition,Data};
    /// let mut db = Db::create("test", "~/.local/rzdb").unwrap();
    /// let table_name = "delete_where";
    /// db.create_or_replace_table(table_name).unwrap();
    /// db.create_column(table_name, "col1").unwrap();
    /// db.create_column(table_name, "col2").unwrap();
    /// db.create_column(table_name, "col3").unwrap();
    /// db.insert(table_name, vec!["1", "2", "3"]).unwrap();
    /// db.insert(table_name, vec!["4", "5", "6"]).unwrap();
    /// db.insert(table_name, vec!["7", "8", "9"]).unwrap();
    ///
    /// db.delete_where(table_name, &[Condition::equal_int("col1", 1)]).unwrap();
    /// let rows = db.select_from(table_name).unwrap();
    /// assert_eq!(rows.len(), 2);
    /// assert_eq!(rows[0].select_at(0).unwrap(), Data::Int(4));
    /// assert_eq!(rows[1].select_at(2).unwrap(), Data::Int(9));
    /// ```
    pub fn delete_where(
        &mut self,
        table_name: &str,
        conditions: &[Condition],
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].delete_where(conditions)?;
        Ok(())
    }

    pub fn delete_row_at(
        &mut self,
        table_name: &str,
        row_idx: usize,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].delete_row(row_idx);
        Ok(())
    }

    pub fn delete_column(
        &mut self,
        table_name: &str,
        column_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].delete_column(column_name)
    }

    pub fn exists(&self, table_name: &str) -> bool {
        self.get_table_id(table_name).is_ok()
    }

    pub fn get_table_id(&self, name: &str) -> Result<usize, Box<dyn Error>> {
        for (idx, table) in self.tables.iter().enumerate() {
            if table.get_name() == name {
                return Ok(idx);
            }
        }
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Table {} not found", name),
        )))
    }

    pub fn store_ids(&mut self, values: Vec<&str>) -> Result<Data, Box<dyn Error>> {
        let data = Data::parse_multi(&values);
        let table_ids = self.get_table_id(".ids")?;
        let mut ids = vec![];
        for datum in &data {
            let new_id = self.tables[table_ids].len();
            ids.push(new_id as i64);
            let line = vec![Data::Int(new_id as i64), Data::Int(1), datum.clone()];
            self.tables[table_ids].insert_data(line)?;
        }
        Ok(Data::Join(Join::new(ids)))
    }

    pub fn from_ids(&self, datum: Data) -> Result<Vec<Data>, Box<dyn Error>> {
        if let Data::Join(join) = datum {
            let table_ids = self.get_table_id(".ids")?;
            let mut result = vec![];
            for id in join.ids {
                let datum = self.tables[table_ids].select_at(2, id as usize);
                result.push(datum?);
            }

            Ok(result)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Data must be a join",
            )))
        }
    }

    fn expand(&self, datum: Data) -> Result<Vec<Data>, Box<dyn Error>> {
        if let Data::Join(join) = datum {
            let mut result = vec![];
            for id in join.ids {
                let datum = self.tables[IDS_TABLE_ID].select_at(IDS_COLUMN_ID, id as usize);
                result.push(datum?);
            }
            Ok(result)
        } else {
            Ok(vec![datum])
        }
    }

    pub fn select_from(&self, table_name: &str) -> Result<Vec<Row>, Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        Ok(self.tables[id].select())
    }

    pub fn select_array(&self, table_name: &str) -> Result<Vec<Vec<Vec<Data>>>, Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        let data = self.tables[id].select();
        let mut result = vec![];
        for row in data {
            let mut row_data = vec![];
            for col in row {
                row_data.push(self.expand(col)?);
            }
            result.push(row_data);
        }
        Ok(result)
    }

    pub fn select_at(
        &self,
        table_name: &str,
        col_idx: usize,
        row_idx: usize,
    ) -> Result<Data, Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].select_at(col_idx, row_idx)
    }

    // create_or_replace() of dest table
    pub fn select_into(
        &mut self,
        dest_table: &str,
        source_table: &str,
        columns: &[&str],
        start_idx: usize,
        end_idx: usize,
    ) -> Result<(), Box<dyn Error>> {
        self.create_or_replace_table(dest_table)?;
        let dest_id = self.get_table_id(dest_table)?;

        for column in columns {
            self.tables[dest_id].create_column(column)?;
        }

        let id = self.get_table_id(source_table)?;
        let table = &self.tables[id];

        //if  rows = table.select_columns(columns)
        let mut rows = vec![];
        if let Ok(r) = table.select_columns(columns) {
            for row in r.iter().skip(start_idx).take(end_idx - start_idx) {
                rows.push(row.clone());
            }
        }
        self.tables[dest_id].append_rows(&mut rows).unwrap();
        Ok(())
    }

    /// Selects columns in the given order from the table and returns a vector of rows.
    /// # Arguments
    /// * `table_name` - The name of the table to select from
    /// * `columns` - The names of the columns to select
    /// # Returns
    /// * `Result<Vec<Row>, Box<dyn Error>>` - A vector of rows containing the selected columns
    /// # Errors
    /// * `Box<dyn Error>` - If the table does not exist or if the columns do not exist
    /// # Examples
    /// ```
    /// use rzdb::Db;
    /// let mut db = Db::create("test", "~/.local/rzdb").unwrap();
    /// db.create_or_replace_table("select_columns").unwrap();
    /// db.create_column("select_columns", "col1").unwrap();
    /// db.create_column("select_columns", "col2").unwrap();
    /// db.create_column("select_columns", "col3").unwrap();
    /// db.insert("select_columns", vec!["1", "2", "3"]).unwrap();
    /// db.insert("select_columns", vec!["4", "5", "6"]).unwrap();
    /// db.insert("select_columns", vec!["7", "8", "9"]).unwrap();
    /// let rows = db.select_columns("select_columns", &["col2", "col3", "col1"]).unwrap();
    /// assert_eq!(rows.len(), 3);
    /// assert_eq!(rows[0].len(), 3);
    /// assert_eq!(rows[0].select_at(0).unwrap().to_string(), "2");
    /// assert_eq!(rows[0].select_at(1).unwrap().to_string(), "3");
    /// assert_eq!(rows[0].select_at(2).unwrap().to_string(), "1");
    /// assert_eq!(rows[1].len(), 3);
    /// assert_eq!(rows[1].select_at(0).unwrap().to_string(), "5");
    /// assert_eq!(rows[1].select_at(1).unwrap().to_string(), "6");
    /// assert_eq!(rows[1].select_at(2).unwrap().to_string(), "4");
    /// assert_eq!(rows[2].len(), 3);
    /// assert_eq!(rows[2].select_at(0).unwrap().to_string(), "8");
    /// assert_eq!(rows[2].select_at(1).unwrap().to_string(), "9");
    /// assert_eq!(rows[2].select_at(2).unwrap().to_string(), "7");
    /// ```
    pub fn select_columns(
        &self,
        table_name: &str,
        columns: &[&str],
    ) -> Result<Vec<Row>, Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].select_columns(columns)
    }

    /// Selects rows from the table that match the given conditions and returns a vector of rows.
    /// # Arguments
    /// * `table_name` - The name of the table to select from
    /// * `conditions` - The conditions to match
    /// # Returns
    /// * `Result<Vec<Row>, Box<dyn Error>>` - A vector of rows containing the selected columns
    /// # Errors
    /// * `Box<dyn Error>` - If the table does not exist or if the columns do not exist
    /// # Examples
    /// ```
    /// use rzdb::Db;
    /// use rzdb::Condition;
    /// let mut db = Db::create("test", "~/.local/rzdb").unwrap();
    /// db.create_or_replace_table("select_where").unwrap();
    /// db.create_column("select_where", "col1").unwrap();
    /// db.create_column("select_where", "col2").unwrap();
    /// db.create_column("select_where", "col3").unwrap();
    /// db.insert("select_where", vec!["1", "2", "3"]).unwrap();
    /// db.insert("select_where", vec!["4", "5", "some_text"]).unwrap();
    /// db.insert("select_where", vec!["7", "8", "9"]).unwrap();
    /// let rows = db.select_where("select_where", &[Condition::equal_int("col2", 5)]).unwrap();
    /// assert_eq!(rows.len(), 1);
    /// assert_eq!(rows[0].len(), 3);
    /// assert_eq!(rows[0].select_at(0).unwrap().to_string(), "4");
    /// let rows = db.select_where("select_where", &[Condition::equal_string("col3", "some_text")]).unwrap();
    /// assert_eq!(rows.len(), 1);
    /// assert_eq!(rows[0].len(), 3);
    /// assert_eq!(rows[0].select_at(1).unwrap().to_string(), "5");
    /// ```
    pub fn select_where(
        &self,
        table_name: &str,
        conditions: &[Condition],
    ) -> Result<Vec<Row>, Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].select_where(conditions)
    }

    pub fn get_column_name_at(
        &self,
        table_name: &str,
        idx: usize,
    ) -> Result<String, Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        Ok(self.tables[id].get_column_name_at(idx))
    }

    pub fn get_column_names(&self, table_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        Ok(self.tables[id].get_column_names())
    }

    pub fn get_row_count(&self, table_name: &str) -> Result<usize, Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        Ok(self.tables[id].row_count())
    }

    pub fn get_column_count(&self, table_name: &str) -> Result<usize, Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        Ok(self.tables[id].column_count())
    }

    pub fn set_at(
        &mut self,
        table_name: &str,
        row_idx: usize,
        column_idx: usize,
        value: Data,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id(table_name)?;
        self.tables[id].set_at(row_idx, column_idx, value)
    }

    pub fn display(&self, table_name: &str) -> Result<String, Box<dyn Error>> {
        let table_id = self.get_table_id(table_name)?;
        let table = &self.tables[table_id];

        let column_names = table.get_column_names();
        let rows = table.select();

        let pad = |s: &str, width: usize| {
            let mut s = s.to_string();
            while s.chars().count() < width {
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
        for column_name in &column_names {
            column_widths.push(column_name.chars().count());
        }
        // get the maximum width of all row values
        for row in &rows {
            for (i, value) in row.select().iter().enumerate() {
                let mut width = column_widths[i];
                if let Ok(data) = self.from_ids(value.clone()) {
                    for datum in &data {
                        width = width.max(datum.to_string().chars().count());
                    }
                } else {
                    let value_str = value.to_string();
                    width = width.max(value_str.chars().count());
                }
                column_widths[i] = width;
            }
        }

        let mut result = String::new();
        // write column names
        for (i, column_name) in column_names.iter().enumerate() {
            let width = column_widths[i];
            result.push_str(&pad(column_name, width + 1));
        }
        result.push('\n');
        // write line under column names
        for (i, _) in column_names.iter().enumerate() {
            let width = column_widths[i];
            result.push_str(&line(width));
            result.push(' ');
        }
        result.push('\n');

        // write row values
        for row in &rows {
            let mut multi_index = 0;
            loop {
                let mut has_more_multi = false;
                for (i, datum) in row.select().iter().enumerate() {
                    if let Ok(multi_data) = self.from_ids(datum.clone()) {
                        if multi_index < multi_data.len() {
                            if multi_index + 1 < multi_data.len() {
                                has_more_multi = true;
                            }
                            result.push_str(&pad(
                                &multi_data[multi_index].to_string(),
                                column_widths[i] + 1,
                            ));
                        } else {
                            result.push_str(&pad("", column_widths[i] + 1));
                        }
                    } else {
                        let width = column_widths[i];
                        if multi_index == 0 {
                            result.push_str(&pad(&datum.to_string(), width + 1));
                        } else {
                            result.push_str(&pad("", width + 1));
                        }
                    }
                }
                result.push('\n');
                if has_more_multi {
                    multi_index += 1;
                } else {
                    break;
                }
            }
        }
        Ok(result)
    }
}
