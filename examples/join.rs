extern crate rzdb;

use rzdb::{Data, Db};

fn main() {
    let db_name = "examples";
    let path = "~/.local/rzdb";
    let table_name = "join";
    println!("Creating new database...");

    let mut db = Db::create(db_name, path).unwrap();
    db.create_table(table_name).unwrap();
    db.create_column(table_name, "datum").unwrap();
    db.create_column(table_name, "value 1").unwrap();
    db.create_column(table_name, "value 2").unwrap();
    db.create_column(table_name, "value 3").unwrap();

    // quickest way to insert data if there is no join
    db.insert(
        table_name,
        vec!["12.02.2022", "regular", "line", "as Vec<&str>"],
    )
    .unwrap();

    // insert data via single items
    let date = Data::parse("12.02.2022");
    let line_1 = Data::parse("regular");
    let line_2 = Data::parse("line");
    let line_3 = Data::parse("as Vec<Data>");
    db.insert_data(table_name, vec![date, line_1, line_2, line_3])
        .unwrap();

    // joins require Data objects; a join is a single Data object containing multiple items
    let date = Data::parse("12.02.2022");
    let multi_item_1 = db.store_ids(vec!["item 1", "item 2", "item 3"]).unwrap();
    let and = Data::parse("and");
    let multi_item_2 = db
        .store_ids(vec![
            "another item 1",
            "another item 2",
            "another item 3",
            "another item 4",
        ])
        .unwrap();
    db.insert_data(table_name, vec![date, multi_item_1, and, multi_item_2])
        .unwrap();

    db.save().unwrap();
    print!("{}", db.display(table_name).unwrap());
    println!("Done.");
}
