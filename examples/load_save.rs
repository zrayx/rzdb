extern crate rzdb;

use rzdb::Db;

fn main() {
    if let Ok(db) = Db::load("load_save", "./db") {
        println!("Loading previously saved database...");
        println!("{}", db.to_string("test1").unwrap());
    } else {
        println!("Creating new database...");
        // note: this will create the directory "./db/load_save" if it does not exist
        let mut db = Db::create("load_save", "./db").unwrap();
        db.create_table("test1").unwrap();
        db.create_column("test1", "name").unwrap();
        db.create_column("test1", "value").unwrap();
        db.insert("test1", vec!["leading quotes", "\"hi there\", said he"])
            .unwrap();
        db.insert("test1", vec!["trailing quotes", "she said: \"hi there!\""])
            .unwrap();
        db.insert("test1", vec!["commas", "hi, there"]).unwrap();
        db.insert("test1", vec!["leading whitespace", "  other words"])
            .unwrap();
        db.insert("test1", vec!["trailing whitespace", "words  "])
            .unwrap();
        db.insert(
            "test1",
            vec!["other", "tab: \t; newline: \n; carriage return: \r"],
        )
        .unwrap();

        println!("Saving new database...");
        db.save().unwrap();
        print!("{}", db.to_string("test1").unwrap());
    }
    println!("Done.");
}
