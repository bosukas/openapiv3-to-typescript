use std::borrow::Cow;

use itertools::Itertools;
use openapiv3::{RefOr, Schema};

use crate::{GeneratorOptions, PartialGeneration};
use crate::schema::generate_schemas;

pub fn generate_schema<'a>(
    schemas: &'a Vec<RefOr<Schema>>,
    options: Cow<'a, GeneratorOptions>,
) -> PartialGeneration<'a> {
    let partial_generation = generate_schemas(schemas, options);
    PartialGeneration {
        typescript: generate_definition(partial_generation.typescript),
        references: partial_generation.references,
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
    use crate::generate::any_of::generate_definition;

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
