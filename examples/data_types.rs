extern crate rzdb;

use rzdb::Db;

fn main() {
    let db_name = "examples";
    let path = "~/.local/rzdb";
    let table_name = "data_types";
    println!("Creating new database...");

    let mut db = Db::create(db_name, path).unwrap();
    db.create_table(table_name).unwrap();

    db.create_column(table_name, "name").unwrap();
    db.create_column(table_name, "value").unwrap();

    db.insert(table_name, vec!["date", "22.2.2022"]).unwrap();
    db.insert(table_name, vec!["date", "22.2."]).unwrap();
    db.insert(table_name, vec!["number", "2200"]).unwrap();

    db.save().unwrap();
    print!("{}", db.to_string(table_name).unwrap());
    println!("Done.");
}
