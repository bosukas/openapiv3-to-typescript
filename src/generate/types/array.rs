use std::borrow::Cow;

use openapiv3::{ArrayType, RefOr, SchemaData, SchemaReference};

use crate::{GeneratorOptions, PartialGeneration};
use crate::schema::generate_schema;

pub fn generate_array<'a, 'b>(
    array_type: &'a ArrayType,
    schema_data: &'a SchemaData,
    options: Cow<'b, GeneratorOptions>,
) -> PartialGeneration<'a> {
    match &array_type.items {
        None => PartialGeneration {
            typescript: Cow::Borrowed("unknown[]"),
            references: vec![],
            read_only: schema_data.read_only,
        },
        Some(item) => match item.as_ref() {
            RefOr::Reference { reference } => match SchemaReference::from_str(reference) {
                SchemaReference::Schema { schema } => PartialGeneration {
                    typescript: Cow::Owned(format!("{}[]", schema)),
                    references: vec![Cow::Borrowed(reference)],
                    read_only: schema_data.read_only,
                },
                SchemaReference::Property { .. } => unimplemented!(),
            },
            RefOr::Item(schema) => {
                let partial_generation = generate_schema(schema, options);
                PartialGeneration {
                    typescript: Cow::Owned(format!("{}[]", partial_generation.typescript)),
                    references: partial_generation.references,
                    read_only: schema_data.read_only,
                }
            }
        },
    }
}
