use std::borrow::Cow;

use openapiv3::ObjectType;

use crate::{GeneratorOptions, PartialGeneration};
use crate::schema::generate_schema_properties;

pub fn generate_object<'a>(
    object_type: &'a ObjectType,
    options: Cow<'a, GeneratorOptions>,
) -> PartialGeneration<'a> {
    let mut partial_generation =
        generate_schema_properties(&object_type.properties, &object_type.required, options);

    let typescript = partial_generation.typescript_mut().to_mut();
    typescript.insert(0, '{');
    typescript.push('}');

    partial_generation
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use openapiv3::{ObjectType, RefOrMap, Schema};

    use crate::generate::types::object::generate_object;
    use crate::GeneratorOptions;

    #[test]
    fn basic_object_type() {
        let mut object_type = ObjectType::default();
        let mut properties = RefOrMap::<Schema>::new();
        properties.insert("name", Schema::new_string());
        properties.insert("age", Schema::new_integer());
        object_type.properties = properties;
        let options = GeneratorOptions::default();

        let result = generate_object(&object_type, Cow::Owned(options));
        let expected_typescript = "{name?:string;age?:number;}";
        let expected_references: Vec<String> = vec![];

        assert_eq!(result.typescript, expected_typescript);
        assert_eq!(result.references, expected_references);
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

        let result = generate_object(&object_type, Cow::Owned(options));
        let expected_typescript = "{name:string;age?:number;}";
        let expected_references: Vec<String> = vec![];

        assert_eq!(result.typescript, expected_typescript);
        assert_eq!(result.references, expected_references);
    }
}
