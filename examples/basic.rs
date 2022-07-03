extern crate rzdb;

use rzdb::Db;

fn main() {
    // note: this will create the directory "./db/basic" if it does not exist
    let mut db = Db::create("basic", "./db").unwrap();
    db.create_table("test1").unwrap();
    db.create_column("test1", "name").unwrap();
    db.create_column("test1", "value").unwrap();
    db.insert("test1", vec!["hello", "world"]).unwrap();
    db.insert("test1", vec!["bon jour", "le monde"]).unwrap();
    db.insert("test1", vec!["你好", "世界"]).unwrap(); // no proper display of double width unicode characters

    print!("{}", db.to_string("test1").unwrap());
}
