mod data;
mod db;
mod row;
mod table;

pub use crate::db::Db;
pub use crate::table::Table;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
