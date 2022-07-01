#[derive(Clone)]
pub enum Data {
    String(String),
    Empty,
    //DbFloat(f64),
    //DbI32(i32),
    //DbDate(Date),
    //DbTime(Time),
}

impl Data {
    pub fn parse(s: &str) -> Data {
        Data::String(s.to_string())
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Data::String(s) => s.clone(),
                Data::Empty => "".to_string(),
            }
        )
    }
}
