use crate::{
    join::Join,
    time::{Date, Time},
};

#[derive(Clone, PartialEq)]
pub enum Data {
    String(String),
    Empty,
    Float(f64),
    Int(i64),
    Date(Date),
    Time(Time),
    Join(Join),
}

fn encode_for_csv(s: &str) -> String {
    let mut out = String::new();
    if s.contains('\n')
        || s.contains('"')
        || s.contains(',')
        || s.contains('\t')
        || s.contains('\r')
        || s.contains('\n')
        || s.starts_with(' ')
        || s.ends_with(' ')
    {
        out.push('"');
        for c in s.chars() {
            match c {
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '"' => out.push_str("\"\""),
                _ => out.push(c),
            }
        }
        out.push('"');
    } else {
        out.push_str(s);
    }

    out
}
struct CsvIterator<'a> {
    s: &'a str,
    pos: usize,
}
impl<'a> CsvIterator<'a> {
    fn new(s: &'a str) -> CsvIterator<'a> {
        CsvIterator { s, pos: 0 }
    }
}
impl<'a> Iterator for CsvIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.pos >= self.s.len() {
            return None;
        }
        let mut out = String::new();
        let mut in_quote = false;
        let mut in_escape = false;
        let mut old_char = 'x';
        let mut num_quotes = 0;

        for c in self.s[self.pos..].chars() {
            self.pos += c.len_utf8();
            if in_escape {
                match c {
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    '"' => out.push('"'),
                    _ => out.push(c),
                }
                in_escape = false;
            } else {
                match c {
                    '"' => {
                        num_quotes += 1;
                        if old_char == '"' && num_quotes % 2 == 1 {
                            out.push('"');
                        } else {
                            in_quote = !in_quote;
                        }
                    }
                    '\\' => {
                        in_escape = true;
                    }
                    '\n' => {
                        if in_quote {
                            out.push(c);
                        } else {
                            return Some(out);
                        }
                    }
                    '\r' => {
                        if in_quote {
                            out.push(c);
                        } else {
                            return Some(out);
                        }
                    }
                    ',' => {
                        if in_quote {
                            out.push(c);
                        } else {
                            return Some(out);
                        }
                    }
                    _ => {
                        out.push(c);
                    }
                }
            }
            old_char = c;
        }
        self.pos += out.len();
        Some(out)
    }
}

impl Data {
    pub fn parse(s: &str) -> Data {
        if s.is_empty() {
            Data::Empty
        } else if let Ok(n) = Join::parse(s) {
            Data::Join(n)
        } else if let Ok(n) = Date::parse(s) {
            Data::Date(n)
        } else if let Ok(n) = Time::parse(s) {
            Data::Time(n)
        } else if let Ok(n) = s.parse::<i64>() {
            Data::Int(n)
        } else if let Ok(n) = s.parse::<f64>() {
            Data::Float(n)
        } else {
            Data::String(s.to_string())
        }
    }

    pub fn parse_multi(v: &[&str]) -> Vec<Data> {
        v.iter().map(|s| Data::parse(*s)).collect()
    }

    pub fn decode_line(s: &str) -> Vec<Data> {
        let mut out = vec![];
        for s in CsvIterator::new(s) {
            out.push(Data::parse(&s));
        }
        out
    }
    pub fn no_time_seconds(&self) -> String {
        match self {
            Data::Time(n) => n.to_string().chars().take(5).collect(),
            _ => self.to_string(),
        }
    }
    pub fn encode_for_csv(&self) -> String {
        match self {
            Data::String(s) => encode_for_csv(s),
            Data::Int(n) => n.to_string(),
            Data::Float(n) => n.to_string(),
            Data::Date(n) => n.to_string(),
            Data::Time(n) => n.to_string(),
            Data::Join(n) => n.to_string(),
            Data::Empty => "".to_string(),
        }
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Data::String(s) => s.to_string(),
                Data::Int(n) => n.to_string(),
                Data::Float(n) => n.to_string(),
                Data::Date(n) => n.to_string(),
                Data::Time(n) => n.to_string(),
                Data::Join(n) => n.to_string(),
                Data::Empty => "".to_string(),
            }
        )
    }
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_for_csv() {
        assert_eq!(encode_for_csv(""), "".to_string());
        assert_eq!(encode_for_csv("a"), "a".to_string());
        assert_eq!(encode_for_csv("a,b"), "\"a,b\"".to_string());
        assert_eq!(encode_for_csv("a\nb"), "\"a\\nb\"".to_string());
        assert_eq!(encode_for_csv("a\"b"), "\"a\"\"b\"".to_string());
    }

    #[test]
    fn test_decode_line() {
        let d = Data::decode_line;
        let s = |a: &str| Data::String(a.to_string());

        assert_eq!(d("a,b,c"), vec![s("a"), s("b"), s("c")]);
        assert_eq!(
            d("\"he said: \"\"hi there!\"\"\""),
            vec![s("he said: \"hi there!\"")]
        );
        assert_eq!(d("commas,\"hi, there\""), vec![s("commas"), s("hi, there")]);
        assert_eq!(d("tab: \\t"), vec![s("tab: \t")]);
        assert_eq!(d("newline: \\n"), vec![s("newline: \n")]);
        assert_eq!(d("carriage return: \\r"), vec![s("carriage return: \r")]);
        assert_eq!(d("tab: \\t, t2: \\t"), vec![s("tab: \t"), s(" t2: \t")]);
    }

    #[test]
    fn test_parse() {
        for d in vec![
            ("1.1", Data::Float(1.1)),
            ("1", Data::Int(1)),
            ("12:24:00", Data::Time(Time::new(12 * 3600 + 24 * 60))),
            ("12:24", Data::Time(Time::new(12 * 3600 + 24 * 60))),
            ("02:24:00", Data::Time(Time::new(2 * 3600 + 24 * 60))),
            ("02:04", Data::Time(Time::new(2 * 3600 + 4 * 60))),
            ("2024-01-01", Data::Date(Date::new(2024, 1, 1))),
            ("1.1.23", Data::Date(Date::new(2023, 1, 1))),
            ("1.1.", Data::Date(Date::new(2022, 1, 1))),
            ("[1,2,3]", Data::Join(Join::from(&[1, 2, 3]))),
            ("[1]", Data::Join(Join::from(&[1]))),
        ] {
            let left = Data::parse(d.0);
            let right = d.1;
            assert_eq!(left, right);
        }
    }

    #[test]
    fn test_display() {
        for d in vec![
            (Data::String("a".to_string()), "a"),
            (Data::Int(1), "1"),
            (Data::Float(1.1), "1.1"),
            (Data::Date(Date::new(2024, 1, 1)), "2024-01-01"),
            (Data::Time(Time::new(2 * 3600 + 4 * 60)), "02:04:00"),
            (Data::Join(Join::from(&[1, 2, 3])), "[1,2,3]"),
            (Data::Join(Join::from(&[1])), "[1]"),
            (Data::Empty, ""),
        ] {
            let left = d.0.to_string();
            let right = d.1;
            assert_eq!(left, right);
        }
    }
}
