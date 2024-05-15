use openapiv3::{ObjectType, RefOr, SchemaData, SchemaReference};

use crate::{parse_schema, Parser};
use crate::partial::PartialTypeScript;

pub fn parse_object(
    parser: &Parser,
    object: &ObjectType,
    _data: &SchemaData,
    required: &Vec<String>,
) -> PartialTypeScript {
    let mut typescript = String::new();
    let mut references = vec![];

    typescript.push('{');

    for (name, property) in &object.properties {
        let is_required = match required.contains(name) {
            true => "",
            false => "?",
        };

        match property {
            RefOr::Reference { reference } => match SchemaReference::from_str(reference) {
                SchemaReference::Schema { ref schema } => {
                    references.push(SchemaReference::from_str(reference));
                    typescript.push_str(&format!("{}{}:{};", name, is_required, schema));
                }
                SchemaReference::Property { .. } => unimplemented!(),
            },
            RefOr::Item(schema) => {
                let parsed_schema = parse_schema(parser, schema);

                for reference in parsed_schema.references {
                    references.push(reference)
                }
                let schema_ts = parsed_schema.ts;
                typescript.push_str(&format!("{}{}:{};", name, is_required, schema_ts));
            }
        }
    }

    typescript.push('}');

    PartialTypeScript {
        ts: typescript,
        references,
    }
}
