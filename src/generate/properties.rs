use std::borrow::Cow;

use openapiv3::{RefOr, RefOrMap, Schema, SchemaReference};

use crate::{GeneratorOptions, PartialGeneration};

pub fn generate_properties<'a, 'b>(
    properties: &'a RefOrMap<Schema>,
    required: &'a Vec<String>,
    read_only: bool,
    options: Cow<'b, GeneratorOptions>,
) -> PartialGeneration<'a> {
    let mut typescript = String::from('{');
    let mut references = vec![];

    for (name, property) in properties {
        let is_required = match required.contains(name) {
            true => "",
            false => "?",
        };

        match property {
            RefOr::Reference { reference } => match SchemaReference::from_str(reference) {
                SchemaReference::Schema { schema } => {
                    typescript.push_str(&format!("{}{}:{};", name, is_required, schema));
                    references.push(reference.into())
                }
                SchemaReference::Property { .. } => unimplemented!(),
            },
            RefOr::Item(item) => {
                let result = crate::schema::generate_schema(item, Cow::Borrowed(&options));

                if options.skip_read_only && result.read_only {
                    continue;
                }

                typescript.push_str(&format!("{}{}:{};", name, is_required, result.typescript));
                references.extend(result.references)
            }
        }
    }

    typescript.push('}');

    PartialGeneration {
        typescript: Cow::Owned(typescript),
        references,
        read_only,
    }
}
