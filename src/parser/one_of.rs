use openapiv3::{RefOr, Schema, SchemaData};

use crate::Parser;
use crate::partial::PartialTypeScript;

pub fn parse_one_of(
    parser: &Parser,
    one_of: &Vec<RefOr<Schema>>,
    _data: &SchemaData,
) -> PartialTypeScript {
    todo!("parse_one_of")
}
