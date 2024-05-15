use openapiv3::{IntegerType, NumberType};

pub fn parse_number(_: NumberType) -> String {
    "number".to_string()
}

pub fn parse_integer(_: IntegerType) -> String {
    "number".to_string()
}
