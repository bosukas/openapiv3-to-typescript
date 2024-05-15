use openapiv3::{ArrayType, ReferenceOr};

use crate::error::ReferenceError;
use crate::parse_schema;
use crate::reference::extract_reference_type;

#[derive(Debug, PartialEq)]
pub struct PartialTypeScript {
    pub typescript: String,
    pub reference: Vec<String>,
}

pub fn parse_array(array_type: ArrayType) -> Result<PartialTypeScript, ReferenceError> {
    match array_type.items {
        None => Ok(PartialTypeScript {
            typescript: "unknown[]".to_string(),
            reference: vec![],
        }),
        Some(reference_or_schema) => match reference_or_schema.unbox() {
            ReferenceOr::Reference { reference } => Ok(PartialTypeScript {
                typescript: format!("{}[]", extract_reference_type(&reference)?),
                reference: vec![reference],
            }),
            ReferenceOr::Item(schema) => {
                let result = parse_schema(schema)?;
                Ok(PartialTypeScript {
                    typescript: format!("{}[]", extract_reference_type(&result.typescript)?),
                    reference: result.references,
                })
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use openapiv3::{ArrayType, ReferenceOr};

    use crate::types::array::{parse_array, PartialTypeScript};

    #[test]
    fn unknown_array() {
        let array_type = ArrayType {
            items: None,
            min_items: None,
            max_items: None,
            unique_items: false,
        };
        let expected = PartialTypeScript {
            typescript: "unknown[]".to_string(),
            reference: vec![],
        };

        assert_eq!(parse_array(array_type), Ok(expected));
    }

    #[test]
    fn known_reference_array() {
        let array_type = ArrayType {
            items: Some(ReferenceOr::Reference {
                reference: "#/components/schemas/Product".to_string(),
            }),
            min_items: None,
            max_items: None,
            unique_items: false,
        };
        let expected = PartialTypeScript {
            typescript: "Product[]".to_string(),
            reference: vec!["#/components/schemas/Product".to_string()],
        };

        assert_eq!(parse_array(array_type), Ok(expected));
    }
}
