use openapiv3::{NumberType, SchemaData};

use crate::Parser;
use crate::partial::PartialTypeScript;

pub fn parse_number(
    _parser: &Parser,
    _number: &NumberType,
    _data: &SchemaData,
) -> PartialTypeScript {
    PartialTypeScript {
        ts: "number".to_string(),
        references: vec![],
    }
}
