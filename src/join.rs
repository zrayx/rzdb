use std::error::Error;

#[derive(Clone, PartialEq)]
pub struct Join {
    v: Vec<i64>,
}

fn gen_error(msg: &str) -> Box<dyn Error> {
    Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, msg))
}

impl Join {
    pub fn from(v: &[i64]) -> Join {
        Join { v: v.to_vec() }
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
        Ok(Join { v: data })
    }
}

impl std::fmt::Display for Join {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, n) in self.v.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", n)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
