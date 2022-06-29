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

    pub fn to_string(&self) -> String {
        match self {
            Data::DbString(s) => s.clone(),
        }
    }
}
