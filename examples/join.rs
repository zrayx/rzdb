extern crate rzdb;

use rzdb::{Data, Db};

fn main() {
    let db_name = "examples";
    let path = "~/.local/rzdb";
    let table_name = "join";
    println!("Creating new database...");

    // date       content
    // 12.02.2022·regular line
    // 12.02.2022┍item 1
    //           ┊item 2
    //           ┕item 3

    let mut db = Db::create(db_name, path).unwrap();
    db.create_table(table_name).unwrap();
    db.create_column(table_name, "name").unwrap();
    db.create_column(table_name, "value").unwrap();

    db.insert(table_name, vec!["12.02.2022", "regular line"])
        .unwrap();
    let date = Data::parse("12.02.2022");
    let multi_line = db.multi_ids(vec!["item 1", "item 2", "item 3"]).unwrap();
    db.insert_data(table_name, vec![date, multi_line]).unwrap();

    db.save().unwrap();
    print!("{}", db.to_string(table_name).unwrap());
    println!("Done.");
}
