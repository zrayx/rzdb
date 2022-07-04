extern crate rzdb;

use rzdb::Db;

fn main() {
    // note: this will create the directory "./db/basic" if it does not exist
    let mut db = Db::create("examples", "~/.local/rzdb").unwrap();
    let table_name = "basic";
    db.create_table(table_name).unwrap();
    db.create_column(table_name, "name").unwrap();
    db.create_column(table_name, "value").unwrap();
    db.insert(table_name, vec!["hello", "world"]).unwrap();
    db.insert(table_name, vec!["bon jour", "le monde"]).unwrap();
    db.insert(table_name, vec!["你好", "世界"]).unwrap(); // no proper display of double width unicode characters

    print!("{}", db.to_string(table_name).unwrap());
}
