use std::borrow::Cow;

use openapiv3::{RefOr, Schema, SchemaData};

use crate::{GeneratorOptions, PartialGeneration};

pub fn generate<'a, 'b>(
    schemas: &'a Vec<RefOr<Schema>>,
    schema_data: &'a SchemaData,
    options: Cow<'b, GeneratorOptions>,
) -> PartialGeneration<'a> {
    todo!()
}
