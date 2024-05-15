use openapiv3::StringType;

pub fn parse_string(string_type: StringType) -> String {
    match string_type.enumeration.is_empty() {
        true => "string".to_string(),
        false => string_type
            .enumeration
            .iter()
            .filter_map(|item| item.clone())
            .map(|item| format!("\"{}\"", item))
            .collect::<Vec<String>>()
            .join("|"),
    }
}

#[cfg(test)]
mod tests {
    use openapiv3::StringType;

    use crate::types::string::parse_string;

    #[test]
    fn enumeration() {
        let string_type = StringType {
            format: Default::default(),
            pattern: None,
            enumeration: vec![
                Some("a".to_string()),
                Some("b".to_string()),
                Some("c".to_string()),
            ],
            min_length: None,
            max_length: None,
        };

        assert_eq!(parse_string(string_type), "\"a\"|\"b\"|\"c\"");
    }

    #[test]
    fn string() {
        let string_type = StringType {
            format: Default::default(),
            pattern: None,
            enumeration: vec![],
            min_length: None,
            max_length: None,
        };

        assert_eq!(parse_string(string_type), "string")
    }
}
