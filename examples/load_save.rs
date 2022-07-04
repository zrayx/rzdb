extern crate rzdb;

use rzdb::Db;

fn main() {
    let db_name = "examples";
    let path = "~/.local/rzdb";
    let table_name = "load_save";
    if let Ok(db) = Db::load(db_name, path) {
        println!("Loading previously saved database...");
        println!("{}", db.to_string(table_name).unwrap());
    } else {
        println!("Creating new database...");
        // note: this will create the directory "./db/load_save" if it does not exist
        let mut db = Db::create(db_name, path).unwrap();
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

        println!("Saving new database...");
        db.save().unwrap();
        print!("{}", db.to_string(table_name).unwrap());
    }
    println!("Done.");
}
