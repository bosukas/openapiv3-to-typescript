use openapiv3::{ReferenceOr, Schema, SchemaKind, Type};

use crate::error::ReferenceError;
use crate::reference::extract_reference_type;
use crate::types::array::parse_array;
use crate::types::boolean::parse_boolean;
use crate::types::number::{parse_integer, parse_number};
use crate::types::object::parse_object;
use crate::types::string::parse_string;

#[derive(Debug, PartialEq)]
pub struct PartialTypeScript {
    pub typescript: String,
    pub references: Vec<String>,
}

pub fn parse_schema(schema: Schema) -> Result<PartialTypeScript, ReferenceError> {
    match schema.schema_kind {
        SchemaKind::Type(Type::String(string_type)) => Ok(PartialTypeScript {
            typescript: parse_string(string_type),
            references: vec![],
        }),
        SchemaKind::Type(Type::Number(number_type)) => Ok(PartialTypeScript {
            typescript: parse_number(number_type),
            references: vec![],
        }),
        SchemaKind::Type(Type::Integer(integer_type)) => Ok(PartialTypeScript {
            typescript: parse_integer(integer_type),
            references: vec![],
        }),
        SchemaKind::Type(Type::Object(object_type)) => {
            let result = parse_object(object_type)?;
            Ok(PartialTypeScript {
                typescript: result.typescript,
                references: result.references,
            })
        }
        SchemaKind::Type(Type::Array(array_type)) => {
            let result = parse_array(array_type)?;

            Ok(PartialTypeScript {
                typescript: result.typescript,
                references: result.reference,
            })
        }
        SchemaKind::Type(Type::Boolean(boolean_type)) => Ok(PartialTypeScript {
            typescript: parse_boolean(boolean_type),
            references: vec![],
        }),
        SchemaKind::Any(any_schema) => {
            let mut typescript = String::new();
            let mut references = vec![];

            for (name, reference_or_schema) in any_schema.properties {
                typescript.push_str(&*format!("{}: ", name));

                match reference_or_schema.unbox() {
                    ReferenceOr::Reference { reference } => {}
                    ReferenceOr::Item(schema) => {
                        let result = parse_schema(schema)?;
                        typescript.push_str(&result.typescript);
                        for reference in result.references {
                            references.push(reference);
                        }
                    }
                }

                typescript.push(';');
            }

            Ok(PartialTypeScript {
                typescript,
                references,
            })
        }
        SchemaKind::OneOf { .. } => {
            unimplemented!("OneOf")
        }
        SchemaKind::AllOf { all_of } => {
            let mut typescripts: Vec<String> = vec![];
            let mut references = vec![];

            for reference_or_schema in all_of {
                match reference_or_schema {
                    ReferenceOr::Reference { reference } => {
                        typescripts.push(extract_reference_type(&reference)?);
                        references.push(reference.clone());
                    }
                    ReferenceOr::Item(schema) => {
                        let result = parse_schema(schema)?;
                        typescripts.push(result.typescript);
                        for reference in result.references {
                            references.push(reference);
                        }
                    }
                }
            }

            Ok(PartialTypeScript {
                typescript: typescripts.join("&"),
                references,
            })
        }
        SchemaKind::AnyOf { any_of } => {
            let mut typescripts: Vec<String> = vec![];
            let mut references = vec![];

            for reference_or_schema in any_of {
                match reference_or_schema {
                    ReferenceOr::Reference { reference } => {
                        typescripts.push(extract_reference_type(&reference)?);
                        references.push(reference.clone());
                    }
                    ReferenceOr::Item(schema) => {
                        let result = parse_schema(schema)?;
                        typescripts.push(result.typescript);
                        for reference in result.references {
                            references.push(reference);
                        }
                    }
                }
            }

            Ok(PartialTypeScript {
                typescript: typescripts.join("|"),
                references,
            })
        }
        SchemaKind::Not { .. } => {
            unimplemented!("Not")
        }
    }
}
