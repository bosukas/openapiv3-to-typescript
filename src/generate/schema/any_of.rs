use std::borrow::Cow;

use itertools::Itertools;
use openapiv3::{RefOr, Schema, SchemaData, SchemaReference};

use crate::{GeneratorOptions, PartialGeneration};

pub fn generate<'a, 'b>(
    schemas: &'a Vec<RefOr<Schema>>,
    schema_data: &'a SchemaData,
    options: Cow<'b, GeneratorOptions>,
) -> PartialGeneration<'a> {
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
                let partial_generation = crate::schema::generate_schema(schema, options.clone());
                for reference in partial_generation.references {
                    references.push(reference);
                }
                typescript.push(partial_generation.typescript)
            }
        }
    }

    PartialGeneration {
        typescript: generate_definition(typescript),
        references,
        read_only: schema_data.read_only,
    }
}

pub fn generate_definition(types: Vec<Cow<str>>) -> Cow<str> {
    let mut result: Vec<Cow<str>> = vec![];
    let n = types.len();

    for t in &types {
        result.push(Cow::Borrowed(t))
    }

    for r in 2..=n {
        for combo in types.iter().combinations(r) {
            let combined = combo.into_iter().join("&");
            result.push(Cow::Owned(format!("({})", combined)))
        }
    }

    Cow::Owned(result.join("|"))
}

#[cfg(test)]
pub mod tests {
    use crate::generate::schema::any_of::generate_definition;

    #[test]
    fn one_type() {
        let values = vec!["a".into()];
        let result = generate_definition(values);
        assert_eq!(result, "a")
    }

    #[test]
    fn two_types() {
        let values = vec!["a".into(), "b".into()];
        let result = generate_definition(values);
        assert_eq!(result, "a|b|(a&b)");
    }

    #[test]
    fn three_types() {
        let values = vec!["a".into(), "b".into(), "c".into()];
        let result = generate_definition(values);
        assert_eq!(result, "a|b|c|(a&b)|(a&c)|(b&c)|(a&b&c)")
    }

    #[test]
    fn four_types() {
        let values = vec!["a".into(), "b".into(), "c".into(), "d".into()];
        let result = generate_definition(values);
        assert_eq!(
            result,
            "a|b|c|d|(a&b)|(a&c)|(a&d)|(b&c)|(b&d)|(c&d)|(a&b&c)|(a&b&d)|(a&c&d)|(b&c&d)|(a&b&c&d)"
        )
    }
}
