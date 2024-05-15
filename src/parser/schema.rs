use openapiv3::{Schema, SchemaKind};

use crate::Parser;
use crate::parser::all_of::parse_all_of;
use crate::parser::any::parse_any;
use crate::parser::any_of::parse_any_of;
use crate::parser::not::parse_not;
use crate::parser::one_of::parse_one_of;
use crate::parser::r#type::parse_type;
use crate::partial::PartialTypeScript;

pub fn parse_schema(parser: &Parser, schema: &Schema) -> PartialTypeScript {
    let required = schema.get_required();
    let empty = vec![];
    let required = match required {
        None => &empty,
        Some(v) => v,
    };
    match &schema.kind {
        SchemaKind::Type(t) => parse_type(parser, t, &schema.data, required),
        SchemaKind::OneOf { one_of } => parse_one_of(parser, one_of, &schema.data),
        SchemaKind::AllOf { all_of } => parse_all_of(parser, all_of, &schema.data),
        SchemaKind::AnyOf { any_of } => parse_any_of(parser, any_of, &schema.data),
        SchemaKind::Not { not } => parse_not(not, parser.oas()),
        SchemaKind::Any(any) => parse_any(parser, any, &schema.data),
    }
}
