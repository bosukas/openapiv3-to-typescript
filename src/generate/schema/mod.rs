use std::borrow::Cow;

use openapiv3::{Schema, SchemaKind};

use crate::{GeneratorOptions, PartialGeneration};

mod all_of;
mod any;
mod any_of;
mod not;
mod one_of;

pub fn generate_schema<'a, 'b>(
    schema: &'a Schema,
    options: Cow<'b, GeneratorOptions>,
) -> PartialGeneration<'a> {
    match &schema.kind {
        SchemaKind::Type(typ) => super::types::generate_type(typ, &schema.data, options),
        SchemaKind::OneOf { one_of } => one_of::generate(one_of, &schema.data, options),
        SchemaKind::AllOf { all_of } => all_of::generate(all_of, &schema.data, options),
        SchemaKind::AnyOf { any_of } => any_of::generate(any_of, &schema.data, options),
        SchemaKind::Not { not } => not::generate(not, &schema.data, options),
        SchemaKind::Any(any) => any::generate(any, &schema.data, options),
    }
}
