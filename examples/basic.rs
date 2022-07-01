extern crate rzdb;

use rzdb::Db;

fn main() {
    // note: this will create the directory "./db/basic" if it does not exist
    let mut db = Db::create("basic", "./db").unwrap();
    db.create_table("test1");
    db.create_column("test1", "name");
    db.create_column("test1", "value");
    db.insert("test1", vec!["hello", "world"]);
    db.insert("test1", vec!["bon jour", "le monde"]);
    db.insert("test1", vec!["你好", "世界"]); // no proper display of double width unicode characters

    print!("{}", db.to_string("test1"));
}
