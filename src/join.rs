use std::error::Error;

#[derive(Clone, PartialEq)]
pub struct Join {
    pub ids: Vec<i64>,
}

fn gen_error(msg: &str) -> Box<dyn Error> {
    Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, msg))
}

impl Join {
    pub fn new(ids: Vec<i64>) -> Join {
        Join { ids }
    }
    pub fn parse(s: &str) -> Result<Join, Box<dyn Error>> {
        let opening_bracket = s.chars().next().unwrap_or('x');
        if opening_bracket != '[' {
            return Err(gen_error("Missing '[' in Join::parse()"));
        }
        let closing_bracket = s.chars().last().unwrap_or('x');
        if closing_bracket != ']' {
            return Err(gen_error("Missing ']' in Join::parse()"));
        }
        let mut data = vec![];
        let l = s.len();
        for s in s[1..(l - 1)].split(',') {
            let n = s.parse::<i64>()?;
            data.push(n);
        }
        Ok(Join { ids: data })
    }
}

impl std::fmt::Display for Join {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, n) in self.ids.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", n)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
