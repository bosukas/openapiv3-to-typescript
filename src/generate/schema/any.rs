use std::borrow::Cow;

use openapiv3::{AnySchema, RefOr, SchemaData, SchemaReference};

use crate::{GeneratorOptions, PartialGeneration};

pub fn generate<'a, 'b>(
    schema: &'a AnySchema,
    schema_data: &'a SchemaData,
    options: Cow<'b, GeneratorOptions>,
) -> PartialGeneration<'a> {
    let mut typescript = String::new();
    let mut references = vec![];
    typescript.insert(0, '{');

    for (name, property) in &schema.properties {
        let is_required = match schema.required.contains(name) {
            true => "",
            false => "?",
        };

        match property {
            RefOr::Reference { reference } => match SchemaReference::from_str(reference) {
                SchemaReference::Schema { schema } => {
                    references.push(reference.into());
                    typescript.push_str(&format!("{}{}:{};", name, is_required, schema));
                }
                SchemaReference::Property { .. } => unimplemented!(),
            },
            RefOr::Item(item) => {
                let result = crate::schema::generate_schema(item, Cow::Borrowed(&options));

                if options.skip_read_only && result.read_only {
                    continue;
                }

                typescript.push_str(&format!("{}{}:{};", name, is_required, result.typescript));
                for reference in result.references {
                    references.push(reference)
                }
            }
        }
    }
    typescript.push('}');

    PartialGeneration {
        typescript: Cow::Owned(typescript),
        references,
        read_only: schema_data.read_only,
    }
}
