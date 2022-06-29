extern crate rzdb;

use rzdb::Db;

fn main() {
    let mut db = Db::create("basic", "./db");
    db.create_table("test1");
    db.create_column("test1", "name");
    db.create_column("test1", "value");
    db.insert("test1", vec!["hello", "world"]);
    db.insert("test1", vec!["bon jour", "le monde"]);
    db.insert("test1", vec!["你好", "世界"]);

    for row in db.select_from("test1") {
        for column in row {
            print!("{},", column.to_string());
        }
        println!();
    }
}
