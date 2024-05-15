use openapiv3::{SchemaData, Type};

use crate::Parser;
use crate::partial::PartialTypeScript;

use self::array::parse_array;
use self::boolean::parse_boolean;
use self::integer::parse_integer;
use self::number::parse_number;
use self::object::parse_object;
use self::string::parse_string;

mod array;
mod boolean;
mod integer;
mod number;
mod object;
mod string;

pub fn parse_type(
    parser: &Parser,
    t: &Type,
    data: &SchemaData,
    required: &Vec<String>,
) -> PartialTypeScript {
    match t {
        Type::String(string) => parse_string(parser, string, data),
        Type::Number(number) => parse_number(parser, number, data),
        Type::Integer(integer) => parse_integer(parser, integer, data),
        Type::Object(object) => parse_object(parser, object, data, required),
        Type::Array(array) => parse_array(parser, array, data),
        Type::Boolean {} => parse_boolean(parser, data),
    }
}
