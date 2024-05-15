use openapiv3::{ObjectType, ReferenceOr};

use crate::error::ReferenceError;
use crate::parse_schema;
use crate::reference::extract_reference_type;

#[derive(Debug, PartialEq)]
pub struct PartialTypeScript {
    pub typescript: String,
    pub references: Vec<String>,
}

pub fn parse_object(object_type: ObjectType) -> Result<PartialTypeScript, ReferenceError> {
    let mut typescript = String::new();
    let mut references = vec![];
    typescript.push('{');

    for (name, property) in object_type.properties {
        typescript.push_str(&*format!("{}:", name));

        match property.unbox() {
            ReferenceOr::Reference { reference } => {
                let type_reference = extract_reference_type(&reference)?;
                typescript.push_str(&type_reference);
                references.push(reference);
            }
            ReferenceOr::Item(schema) => {
                let result = parse_schema(schema)?;
                for reference in result.references {
                    references.push(reference)
                }
                typescript.push_str(&result.typescript);
            }
        }

        //match property {
        //    ReferenceOr::Reference { reference } => {
        //        let type_reference = extract_reference_type(&reference)?;
        //        typescript.push_str(&type_reference);
        //        references.push(reference);
        //    }
        //    ReferenceOr::Item(schema) => match schema.schema_kind {
        //        SchemaKind::Type(Type::String(string_type)) => {
        //            typescript.push_str(&parse_string(string_type));
        //        }
        //        SchemaKind::Type(Type::Number(number_type)) => {
        //            typescript.push_str(&parse_number(number_type))
        //        }
        //        SchemaKind::Type(Type::Integer(integer_type)) => {
        //            typescript.push_str(&parse_integer(integer_type))
        //        }
        //        SchemaKind::Type(Type::Object(object_type)) => {
        //            let result = parse_object(object_type)?;
        //            typescript.push_str(&result.typescript);
        //            for reference in result.references {
        //                references.push(reference)
        //            }
        //        }
        //        SchemaKind::Type(Type::Array(array_type)) => {
        //            let result = parse_array(array_type)?;
        //            typescript.push_str(&result.typescript);
        //            for reference in result.reference {
        //                references.push(reference)
        //            }
        //        }
        //        SchemaKind::Type(Type::Boolean(boolean_type)) => {
        //            typescript.push_str(&parse_boolean(boolean_type))
        //        }
        //        _ => unimplemented!(),
        //    },
        //}

        typescript.push(';');
    }

    typescript.push_str("}");

    Ok(PartialTypeScript {
        typescript,
        references,
    })
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use openapiv3::{ObjectType, ReferenceOr, Schema, SchemaKind, Type};

    use crate::types::object::{parse_object, PartialTypeScript};

    #[test]
    fn it_works() {
        let mut map = IndexMap::new();
        map.insert(
            "male".to_string(),
            ReferenceOr::Item(Box::new(Schema {
                schema_data: Default::default(),
                schema_kind: SchemaKind::Type(Type::Boolean(Default::default())),
            })),
        );
        map.insert(
            "age".to_string(),
            ReferenceOr::Item(Box::new(Schema {
                schema_data: Default::default(),
                schema_kind: SchemaKind::Type(Type::Number(Default::default())),
            })),
        );
        map.insert(
            "name".to_string(),
            ReferenceOr::Item(Box::new(Schema {
                schema_data: Default::default(),
                schema_kind: SchemaKind::Type(Type::String(Default::default())),
            })),
        );
        map.insert(
            "address".to_string(),
            ReferenceOr::Reference {
                reference: "#/components/schemas/Address".to_string(),
            },
        );

        let object_type = ObjectType {
            properties: map,
            required: vec![],
            additional_properties: None,
            min_properties: None,
            max_properties: None,
        };

        let expected = PartialTypeScript {
            typescript: "{male:boolean;age:number;name:string;address:Address;}".to_string(),
            references: vec!["#/components/schemas/Address".to_string()],
        };

        assert_eq!(parse_object(object_type), Ok(expected));
    }
}
