use std::borrow::Cow;

use openapiv3::{ObjectType, SchemaData};

use crate::{GeneratorOptions, PartialGeneration};

pub fn generate_object<'a, 'b>(
    object_type: &'a ObjectType,
    schema_data: &'a SchemaData,
    options: Cow<'b, GeneratorOptions>,
) -> PartialGeneration<'a> {
    crate::generate::properties::generate_properties(
        &object_type.properties,
        &object_type.required,
        schema_data.read_only,
        options,
    )
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use openapiv3::{ObjectType, RefOrMap, Schema, SchemaData};

    use crate::generate::types::object::generate_object;
    use crate::GeneratorOptions;

    #[test]
    fn basic_object_type() {
        let mut object_type = ObjectType::default();
        let schema_data = SchemaData::default();
        let mut properties = RefOrMap::<Schema>::new();
        properties.insert("name", Schema::new_string());
        properties.insert("age", Schema::new_integer());
        object_type.properties = properties;
        let options = GeneratorOptions::default();

        let result = generate_object(&object_type, &schema_data, Cow::Owned(options));
        let expected_typescript = "{name?:string;age?:number;}";
        let expected_references: Vec<String> = vec![];

        assert_eq!(result.typescript, expected_typescript);
        assert_eq!(result.references, expected_references);
    }

    #[test]
    fn required_property() {
        let mut object_type = ObjectType::default();
        let schema_data = SchemaData::default();
        let mut properties = RefOrMap::<Schema>::new();
        properties.insert("name", Schema::new_string());
        properties.insert("age", Schema::new_integer());
        object_type.properties = properties;
        object_type.required = vec!["name".to_string()];
        let options = GeneratorOptions::default();

        let result = generate_object(&object_type, &schema_data, Cow::Owned(options));
        let expected_typescript = "{name:string;age?:number;}";
        let expected_references: Vec<String> = vec![];

        assert_eq!(result.typescript, expected_typescript);
        assert_eq!(result.references, expected_references);
    }
}
