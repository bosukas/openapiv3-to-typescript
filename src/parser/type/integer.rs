use openapiv3::{IntegerType, SchemaData};

use crate::Parser;
use crate::partial::PartialTypeScript;

pub fn parse_integer(
    _parser: &Parser,
    _integer: &IntegerType,
    _data: &SchemaData,
) -> PartialTypeScript {
    PartialTypeScript {
        ts: "number".to_string(),
        references: vec![],
    }
}
