#[derive(Clone, PartialEq)]
pub enum Data {
    String(String),
    Empty,
    //Float(f64),
    //I64(i64),
    //Date(Date),
    //Time(Time),
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

// iterator for decoding csv
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

        for c in self.s.chars().skip(self.pos) {
            self.pos += 1;
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
        Data::String(s.to_string())
    }

    pub fn decode_line(s: &str) -> Vec<Data> {
        let mut out = vec![];
        for s in CsvIterator::new(s) {
            out.push(Data::parse(&s));
        }
        out
    }

    pub fn encode_for_csv(&self) -> String {
        match self {
            Data::String(s) => encode_for_csv(s),
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
                Data::String(s) => s.clone(),
                Data::Empty => "".to_string(),
            }
        )
    }
}

impl std::fmt::Debug for Data {
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
}
