use openapiv3::{RefOr, Schema, SchemaData, SchemaReference};

use crate::{parse_schema, Parser};
use crate::partial::PartialTypeScript;

pub fn parse_any_of(
    parser: &Parser,
    any_of: &Vec<RefOr<Schema>>,
    _data: &SchemaData,
) -> PartialTypeScript {
    let mut typescript = vec![];
    let mut references = vec![];

    for reference_or_schema in any_of {
        match reference_or_schema {
            RefOr::Reference { reference } => {
                let schema_reference = SchemaReference::from_str(reference);
                match schema_reference {
                    SchemaReference::Schema { ref schema } => {
                        typescript.push(schema.clone());
                    }
                    SchemaReference::Property { .. } => unimplemented!(),
                }
                references.push(schema_reference);
            }
            RefOr::Item(schema) => {
                let parsed_schema = parse_schema(parser, schema);
                typescript.push(parsed_schema.ts);
                for reference in parsed_schema.references {
                    references.push(reference);
                }
            }
        }
    }

    PartialTypeScript {
        ts: typescript.join("|"),
        references,
    }
}
