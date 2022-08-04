extern crate rzdb;

use rzdb::Db;

fn main() {
    let db_name = "examples";
    let db_dir = "~/.local/rzdb";
    let table_name = "load_save";

    println!("Creating new database...");
    println!("========================");
    // note: this will create the directory "~/.local/rzdb/examples/load_save" if it does not exist
    let mut db = Db::create(db_name, db_dir).unwrap();
    db.create_table(table_name).unwrap();
    db.create_column(table_name, "name").unwrap();
    db.create_column(table_name, "value").unwrap();
    db.insert(table_name, vec!["leading quotes", "\"hi there\", said he"])
        .unwrap();
    db.insert(
        table_name,
        vec!["trailing quotes", "she said: \"hi there!\""],
    )
    .unwrap();
    db.insert(table_name, vec!["commas", "hi, there"]).unwrap();
    db.insert(table_name, vec!["leading whitespace", "  other words"])
        .unwrap();
    db.insert(table_name, vec!["trailing whitespace", "words  "])
        .unwrap();
    db.insert(
        table_name,
        vec!["other", "tab: \t; newline: \n; carriage return: \r"],
    )
    .unwrap();
    print!("{}", db.to_string(table_name).unwrap());

    println!();
    println!("Saving, then loading database...");
    println!("================================");
    db.save().unwrap();
    match Db::load(db_name, db_dir) {
        Ok(db) => println!("{}", db.display(table_name).unwrap()),
        Err(e) => println!("Failed to load database: {}", e),
    };

    println!("Dropping table...");
    println!("=================");
    db.drop_table(table_name).unwrap();

    println!("Done.");
}
