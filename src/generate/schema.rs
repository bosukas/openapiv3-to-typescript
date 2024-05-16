use std::borrow::Cow;

use openapiv3::{AnySchema, RefOr, RefOrMap, Schema, SchemaKind, SchemaReference};

use crate::{GeneratorOptions, PartialGeneration, PartialVecGeneration};

pub fn generate_schema<'a>(
    schema: &'a Schema,
    options: Cow<'a, GeneratorOptions>,
) -> PartialGeneration<'a> {
    match &schema.kind {
        SchemaKind::Type(typ) => super::types::generate_type(typ, &schema.data, options),
        SchemaKind::OneOf { one_of } => {
            let partial_generation = generate_schemas(one_of, options);
            PartialGeneration {
                typescript: Cow::Owned(partial_generation.typescript.join("|")),
                references: partial_generation.references,
            }
        }
        SchemaKind::AllOf { all_of } => {
            let partial_generation = generate_schemas(all_of, options);
            PartialGeneration {
                typescript: Cow::Owned(partial_generation.typescript.join("&")),
                references: partial_generation.references,
            }
        }
        SchemaKind::AnyOf { any_of } => super::any_of::generate_schema(any_of, options),
        SchemaKind::Not { .. } => {
            unimplemented!("not")
        }
        SchemaKind::Any(any) => generate_any_schema(any, options),
    }
}

pub fn generate_schemas<'a>(
    schemas: &'a Vec<RefOr<Schema>>,
    options: Cow<'a, GeneratorOptions>,
) -> PartialVecGeneration<'a> {
    let mut typescript: Vec<Cow<str>> = vec![];
    let mut references: Vec<Cow<str>> = vec![];

    for element in schemas {
        match element {
            RefOr::Reference { reference } => match SchemaReference::from_str(reference) {
                SchemaReference::Schema { schema } => {
                    typescript.push(Cow::Owned(schema));
                    references.push(Cow::Borrowed(reference))
                }
                SchemaReference::Property { .. } => unimplemented!(),
            },
            RefOr::Item(schema) => {
                let partial_generation = generate_schema(schema, options.clone());
                for reference in partial_generation.references {
                    references.push(reference);
                }
                typescript.push(partial_generation.typescript)
            }
        }
    }

    PartialVecGeneration {
        typescript,
        references,
    }
}

pub fn generate_any_schema<'a>(
    schema: &'a AnySchema,
    options: Cow<'a, GeneratorOptions>,
) -> PartialGeneration<'a> {
    let mut partial_generation =
        generate_schema_properties(&schema.properties, &schema.required, options);

    let typescript = partial_generation.typescript_mut().to_mut();
    typescript.insert(0, '{');
    typescript.push('}');

    partial_generation
}

pub fn generate_schema_properties<'a>(
    properties: &'a RefOrMap<Schema>,
    required: &'a Vec<String>,
    options: Cow<'a, GeneratorOptions>,
) -> PartialGeneration<'a> {
    let mut typescript = String::new();
    let mut references: Vec<Cow<str>> = vec![];

    for (name, property) in properties {
        let is_required = match required.contains(name) {
            true => "",
            false => "?",
        };

        let property_typescript = match property {
            RefOr::Reference { reference } => {
                references.push(Cow::Borrowed(reference));
                match SchemaReference::from_str(reference) {
                    SchemaReference::Schema { schema } => Cow::Owned(schema),
                    SchemaReference::Property { .. } => unimplemented!(),
                }
            }
            RefOr::Item(schema) => {
                let partial_generation = generate_schema(schema, options.clone());
                for reference in partial_generation.references {
                    references.push(reference)
                }
                partial_generation.typescript
            }
        };

        typescript.push_str(&format!("{}{}:{};", name, is_required, property_typescript))
    }

    PartialGeneration {
        typescript: Cow::Owned(typescript),
        references,
    }
}
