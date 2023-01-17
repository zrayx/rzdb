use crate::data::Data;

pub struct Condition<'a> {
    pub column: &'a str,
    pub data: Data,
    pub condition: ConditionType,
}

impl Condition<'_> {
    pub fn new(column: &str, data: Data, condition: ConditionType) -> Condition {
        Condition {
            column,
            data,
            condition,
        }
    }

    pub fn equal_string<'a>(column: &'a str, data: &str) -> Condition<'a> {
        Condition::new(column, Data::String(data.to_string()), ConditionType::Equal)
    }

    pub fn equal_int(column: &str, data: i64) -> Condition {
        Condition::new(column, Data::Int(data), ConditionType::Equal)
    }

    pub fn matches(&self, data: &Data) -> bool {
        match self.condition {
            ConditionType::Equal => self.data == *data,
            _ => unimplemented!(),
        }
    }
}

pub enum ConditionType {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}
