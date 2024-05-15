use openapiv3::{ArrayType, RefOr, SchemaData, SchemaReference};

use crate::{parse_schema, Parser};
use crate::partial::PartialTypeScript;

pub fn parse_array(parser: &Parser, array: &ArrayType, _data: &SchemaData) -> PartialTypeScript {
    match &array.items {
        None => PartialTypeScript {
            ts: "unknown[]".to_string(),
            references: vec![],
        },
        Some(ref_or_schema) => match ref_or_schema.as_ref() {
            RefOr::Reference { reference } => {
                let schema_reference = SchemaReference::from_str(reference);

                match schema_reference {
                    SchemaReference::Schema { ref schema } => PartialTypeScript {
                        ts: format!("{}[]", schema),
                        references: vec![schema_reference],
                    },
                    SchemaReference::Property { .. } => unimplemented!(),
                }
            }
            RefOr::Item(schema) => {
                let parsed_schema = parse_schema(parser, schema);

                PartialTypeScript {
                    ts: format!("{}[]", parsed_schema.ts),
                    references: parsed_schema.references,
                }
            }
        },
    }
}
