use std::borrow::Cow;

use openapiv3::{ObjectType, RefOr, SchemaData, SchemaReference};

use crate::{GeneratorOptions, PartialGeneration};

pub fn generate_object<'a, 'b>(
    object_type: &'a ObjectType,
    schema_data: &'a SchemaData,
    options: Cow<'b, GeneratorOptions>,
) -> PartialGeneration<'a> {
    let mut typescript = String::new();
    let mut references = vec![];
    typescript.insert(0, '{');

    for (name, property) in &object_type.properties {
        let is_required = match object_type.required.contains(name) {
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

#[cfg(test)]
mod tests {
    use openapiv3::{ObjectType, RefOrMap, Schema};

    use crate::GeneratorOptions;

    #[test]
    fn basic_object_type() {
        let mut object_type = ObjectType::default();
        let mut properties = RefOrMap::<Schema>::new();
        properties.insert("name", Schema::new_string());
        properties.insert("age", Schema::new_integer());
        object_type.properties = properties;
        let options = GeneratorOptions::default();

        //let result = generate_object(&object_type, Cow::Owned(options));
        //let expected_typescript = "{name?:string;age?:number;}";
        //let expected_references: Vec<String> = vec![];
        //
        //assert_eq!(result.typescript, expected_typescript);
        //assert_eq!(result.references, expected_references);
    }

    #[test]
    fn required_property() {
        let mut object_type = ObjectType::default();
        let mut properties = RefOrMap::<Schema>::new();
        properties.insert("name", Schema::new_string());
        properties.insert("age", Schema::new_integer());
        object_type.properties = properties;
        object_type.required = vec!["name".to_string()];
        let options = GeneratorOptions::default();

        //let result = generate_object(&object_type, Cow::Owned(options));
        //let expected_typescript = "{name:string;age?:number;}";
        //let expected_references: Vec<String> = vec![];
        //
        //assert_eq!(result.typescript, expected_typescript);
        //assert_eq!(result.references, expected_references);
    }
}
