use std::borrow::Cow;

use openapiv3::{RefOr, Schema, SchemaData, SchemaReference};

use crate::{GeneratorOptions, PartialGeneration};

pub fn generate<'a, 'b>(
    schemas: &'a Vec<RefOr<Schema>>,
    _schema_data: &'a SchemaData,
    options: Cow<'b, GeneratorOptions>,
) -> PartialGeneration<'a> {
    let mut typescript: Vec<Cow<str>> = vec![];
    let mut references: Vec<Cow<str>> = vec![];
    let mut read_only = false;

    for e in schemas {
        match e {
            RefOr::Reference { reference } => {
                match SchemaReference::from_str(reference) {
                    SchemaReference::Schema { schema } => {
                        references.push(reference.into());
                        typescript.push(schema.into());
                    }
                    SchemaReference::Property { .. } => unimplemented!(),
                };
            }
            RefOr::Item(item) => {
                let result = crate::schema::generate_schema(item, Cow::Borrowed(&options));

                for reference in result.references {
                    references.push(reference)
                }
                typescript.push(result.typescript);
                read_only = result.read_only;
            }
        }
    }
    PartialGeneration {
        typescript: Cow::Owned(typescript.join("&")),
        references,
        read_only,
    }
}
