use std::borrow::Cow;

use openapiv3::StringType;

pub fn generate_string(string_type: &StringType) -> Cow<str> {
    match string_type.enumeration.is_empty() {
        true => Cow::Borrowed("string"),
        false => generate_enumeration(&string_type.enumeration),
    }
}

fn generate_enumeration(values: &Vec<String>) -> Cow<str> {
    let enumeration = values
        .iter()
        .map(|item| format!("\"{}\"", item))
        .collect::<Vec<String>>()
        .join("|");
    Cow::Owned(enumeration)
}

#[cfg(test)]
mod tests {
    use openapiv3::StringType;

    use crate::generate::types::string::{generate_enumeration, generate_string};

    #[test]
    fn generate_enum() {
        let values = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        assert_eq!(generate_enumeration(&values), "\"a\"|\"b\"|\"c\"|\"d\"")
    }

    #[test]
    fn generic_string_type() {
        let string_type = StringType::default();
        assert_eq!(generate_string(&string_type), "string")
    }

    #[test]
    fn enum_string_type() {
        let mut string_type = StringType::default();
        string_type.enumeration = vec![
            "a".to_string(),
            "c".to_string(),
            "e".to_string(),
            "g".to_string(),
        ];
        assert_eq!(generate_string(&string_type), "\"a\"|\"c\"|\"e\"|\"g\"")
    }
}
