use std::error::Error;

use crate::data::Data;
use crate::table::Table;
use crate::time::Timestamp;

pub struct Db {
    pub name: String,
    pub full_path: String,
    pub backup_path: String,
    tables: Vec<Table>,
}

impl Db {
    fn path_names(name: &str, path: &str) -> (String, String) {
        let full_path = format!("{}/{}", path, name);
        let backup_path = format!("{}/{}/backup", path, name);
        (full_path, backup_path)
    }

    pub fn create(name: &str, path: &str) -> Result<Db, Box<dyn Error>> {
        // create path
        let (full_path, backup_path) = Db::path_names(name, path);
        std::fs::create_dir_all(&full_path).unwrap();
        std::fs::create_dir_all(&backup_path).unwrap();

        Ok(Db {
            name: name.to_string(),
            full_path,
            backup_path,
            tables: vec![],
        })
    }

    pub fn load(name: &str, path: &str) -> Result<Db, Box<dyn Error>> {
        let (full_path, backup_path) = Db::path_names(name, path);

        let mut db = Db {
            name: name.to_string(),
            full_path,
            backup_path,
            tables: vec![],
        };

        // load each file in the directory
        for entry in std::fs::read_dir(&db.full_path)? {
            let entry = entry?;
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let full_filename = format!("{}/{}", &db.full_path, filename);
            if filename.ends_with(".csv") {
                println!("a1 filename: {}, db_path: {}", filename, &db.full_path);
                let table = Table::load(&full_filename)?;
                println!("a2");
                db.tables.push(table);
            }
        }
        Ok(db)
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        for table in &mut self.tables {
            if table.is_changed() {
                let filename = format!("{}/{}.csv", self.full_path, table.name);

                let timestamp = Timestamp::now().to_filename_string();
                let backup_filename =
                    format!("{}/{}-{}.csv", self.backup_path, table.name, timestamp);
                std::fs::copy(&filename, &backup_filename)?;

                table.save(&filename)?;
            }
        }

        Ok(())
    }

    pub fn create_table(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        for table in &mut self.tables {
            if table.name == name {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "Table already exists",
                )));
            }
        }
        self.tables.push(Table::create(name));
        Ok(())
    }

    pub fn create_column(
        &mut self,
        table_name: &str,
        column_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        self.tables[id].create_column(column_name)
    }

    pub fn rename_column(
        &mut self,
        table_name: &str,
        old_name: &str,
        new_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        self.tables[id].rename_column(old_name, new_name)
    }

    pub fn insert_column_at(
        &mut self,
        table_name: &str,
        column_name: &str,
        index: usize,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        self.tables[id].insert_column_at(column_name, index);
        Ok(())
    }

    pub fn insert_row_at(&mut self, table_name: &str, index: usize) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        self.tables[id].insert_row_at(index);
        Ok(())
    }

    pub fn delete_row_at(
        &mut self,
        table_name: &str,
        row_idx: usize,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        self.tables[id].delete_row(row_idx);
        Ok(())
    }

    pub fn delete_column(
        &mut self,
        table_name: &str,
        column_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        self.tables[id].delete_column(column_name)
    }

    pub fn exists(&self, table_name: &str) -> bool {
        self.get_table_id(table_name).is_some()
    }

    pub fn get_table_id_result(&self, name: &str) -> Result<usize, Box<dyn Error>> {
        for (idx, table) in self.tables.iter().enumerate() {
            if table.name == name {
                return Ok(idx);
            }
        }
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Table {} not found", name),
        )))
    }

    pub fn get_table_id(&self, name: &str) -> Option<usize> {
        for (idx, table) in self.tables.iter().enumerate() {
            if table.name == name {
                return Some(idx);
            }
        }
        None
    }

    pub fn insert(&mut self, table_name: &str, values: Vec<&str>) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        self.tables[id].insert(values)
    }

    pub fn select_from(&self, table_name: &str) -> Result<Vec<Vec<Data>>, Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        Ok(self.tables[id].select())
    }

    pub fn select_at(
        &self,
        table_name: &str,
        col_idx: usize,
        row_idx: usize,
    ) -> Result<Data, Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        Ok(self.tables[id].select_at(col_idx, row_idx))
    }

    pub fn to_string(&self, table_name: &str) -> Result<String, Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        Ok(self.tables[id].to_string())
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_column_name_at(
        &self,
        table_name: &str,
        idx: usize,
    ) -> Result<String, Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        Ok(self.tables[id].get_column_name_at(idx))
    }

    pub fn get_column_names(&self, table_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        Ok(self.tables[id].get_column_names())
    }

    pub fn get_row_count(&self, table_name: &str) -> Result<usize, Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        Ok(self.tables[id].row_count())
    }

    pub fn get_column_count(&self, table_name: &str) -> Result<usize, Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        Ok(self.tables[id].column_count())
    }

    pub fn set_at(
        &mut self,
        table_name: &str,
        row_idx: usize,
        column_idx: usize,
        value: Data,
    ) -> Result<(), Box<dyn Error>> {
        let id = self.get_table_id_result(table_name)?;
        self.tables[id].set_at(row_idx, column_idx, value)
    }
}
