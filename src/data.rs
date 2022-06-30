#[derive(Clone)]
pub enum Data {
    DbString(String),
    //DbFloat(f64),
    //DbI32(i32),
    //DbDate(Date),
    //DbTime(Time),
}

impl Data {
    pub fn parse(s: &str) -> Data {
        Data::DbString(s.to_string())
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Data::DbString(s) => s.clone(),
            }
        )
    }
}
