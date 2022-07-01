use crate::data::Data;
use crate::table::Table;
use std::error::Error;

pub struct Db {
    pub name: String,
    pub path: String,
    tables: Vec<Table>,
}

impl Db {
    pub fn create(name: &str, path: &str) -> Result<Db, Box<dyn Error>> {
        // create path
        let db_path = format!("{}/{}", path, name);
        std::fs::create_dir_all(db_path).unwrap();

        Ok(Db {
            name: name.to_string(),
            path: path.to_string(),
            tables: vec![],
        })
    }

    pub fn load(name: &str, path: &str) -> Result<Db, Box<dyn Error>> {
        let mut db = Db {
            name: name.to_string(),
            path: path.to_string(),
            tables: vec![],
        };

        let db_path = format!("{}/{}", path, name);

        // load each file in the directory
        for entry in std::fs::read_dir(&db_path)? {
            let entry = entry?;
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let full_filename = format!("{}/{}", db_path, filename);
            if filename.ends_with(".csv") {
                println!("a1 filename: {}, db_path: {}", filename, db_path);
                let table = Table::load(&full_filename)?;
                println!("a2");
                db.tables.push(table);
            }
        }
        Ok(db)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let db_path = format!("{}/{}", self.path, self.name);

        for table in &self.tables {
            let filename = format!("{}/{}.csv", db_path, table.name);
            let mut out = String::new();
            for (idx, name) in table.get_column_names().iter().enumerate() {
                if idx > 0 {
                    out.push(',');
                }
                out.push_str(name);
            }
            out.push('\n');

            let all_data = table.select();
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
        }

        Ok(())
    }

    pub fn create_table(&mut self, name: &str) {
        self.tables.push(Table::create(name));
    }

    pub fn create_column(&mut self, table_name: &str, column_name: &str) {
        if let Some(id) = self.get_table_id(table_name) {
            self.tables[id].create_column(column_name);
        } else {
            panic!(
                "Db::create_column {}: could not find table {}",
                column_name, table_name
            );
        }
    }

    pub fn exists(&self, table_name: &str) -> bool {
        self.get_table_id(table_name).is_some()
    }

    pub fn get_table_id(&self, name: &str) -> Option<usize> {
        for (idx, table) in self.tables.iter().enumerate() {
            if table.name == name {
                return Some(idx);
            }
        }
        None
    }

    pub fn insert(&mut self, table_name: &str, values: Vec<&str>) {
        if let Some(id) = self.get_table_id(table_name) {
            self.tables[id].insert(values);
        } else {
            panic!(
                "Db::insert({}, {:?}): could not find table",
                table_name, values
            );
        }
    }

    pub fn select_from(&self, table_name: &str) -> Vec<Vec<Data>> {
        if let Some(id) = self.get_table_id(table_name) {
            self.tables[id].select()
        } else {
            panic!("Db::select_from({}): could not find table", table_name);
        }
    }

    pub fn to_string(&self, table_name: &str) -> String {
        if let Some(id) = self.get_table_id(table_name) {
            self.tables[id].to_string()
        } else {
            panic!("Db::to_string({}): could not find table", table_name);
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_column_names(&self, table_name: &str) -> Vec<String> {
        if let Some(id) = self.get_table_id(table_name) {
            self.tables[id].get_column_names()
        } else {
            panic!("Db::get_column_names({}): could not find table", table_name);
        }
    }

    pub fn row_count(&self, table_name: &str) -> usize {
        if let Some(id) = self.get_table_id(table_name) {
            self.tables[id].row_count()
        } else {
            panic!("Db::row_count({}): could not find table", table_name);
        }
    }

    pub fn column_count(&self, table_name: &str) -> usize {
        if let Some(id) = self.get_table_id(table_name) {
            self.tables[id].column_count()
        } else {
            panic!("Db::column_count({}): could not find table", table_name);
        }
    }

    pub fn set_at(
        &mut self,
        table_name: &str,
        row_idx: usize,
        column_idx: usize,
        value: Data,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(id) = self.get_table_id(table_name) {
            self.tables[id].set_at(row_idx, column_idx, value)
        } else {
            panic!("Db::column_count({}): could not find table", table_name);
        }
    }
}
