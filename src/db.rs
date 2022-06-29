use crate::data::Data;
use crate::table::Table;

pub struct Db {
    pub name: String,
    pub path: String,
    tables: Vec<Table>,
}

impl Db {
    pub fn create(name: &str, path: &str) -> Db {
        Db {
            name: name.to_string(),
            path: path.to_string(),
            tables: vec![],
        }
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

    pub fn select_from(&mut self, table_name: &str) -> Vec<Vec<Data>> {
        if let Some(id) = self.get_table_id(table_name) {
            self.tables[id].select()
        } else {
            panic!("Db::select_from({}): could not find table", table_name);
        }
    }
}
