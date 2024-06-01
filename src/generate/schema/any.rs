use std::borrow::Cow;

use openapiv3::{AnySchema, SchemaData};

use crate::{GeneratorOptions, PartialGeneration};

pub fn generate<'a, 'b>(
    schema: &'a AnySchema,
    schema_data: &'a SchemaData,
    options: Cow<'b, GeneratorOptions>,
) -> PartialGeneration<'a> {
    crate::generate::properties::generate_properties(
        &schema.properties,
        &schema.required,
        schema_data.read_only,
        options,
    )
}
