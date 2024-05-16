use std::borrow::Cow;

use openapiv3::{SchemaData, Type};

use crate::{GeneratorOptions, PartialGeneration};

mod array;
mod boolean;
mod integer;
mod number;
mod object;
mod string;

pub fn generate_type<'a, 'b>(
    typ: &'a Type,
    schema_data: &'a SchemaData,
    options: Cow<'b, GeneratorOptions>,
) -> PartialGeneration<'a> {
    let mut partial_generation = match typ {
        Type::String(string_type) => PartialGeneration {
            typescript: string::generate_string(string_type),
            references: vec![],
            read_only: schema_data.read_only,
        },
        Type::Number(number_type) => PartialGeneration {
            typescript: number::generate_number(number_type),
            references: vec![],
            read_only: schema_data.read_only,
        },
        Type::Integer(integer_type) => PartialGeneration {
            typescript: integer::generate_integer(integer_type),
            references: vec![],
            read_only: schema_data.read_only,
        },
        Type::Object(object_type) => object::generate_object(object_type, schema_data, options),
        Type::Array(array_type) => array::generate_array(array_type, schema_data, options),
        Type::Boolean {} => PartialGeneration {
            typescript: boolean::generate_boolean(),
            references: vec![],
            read_only: schema_data.read_only,
        },
    };

    if schema_data.nullable {
        let typescript = partial_generation.typescript_mut().to_mut();

        typescript.insert(0, '(');
        typescript.push_str("|null)");
    };
    partial_generation
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use openapiv3::{SchemaData, StringType, Type};

    use crate::generate::types::generate_type;
    use crate::GeneratorOptions;

    #[test]
    fn nullable_string() {
        let typ = Type::String(StringType::default());
        let mut schema_data = SchemaData::default();
        schema_data.nullable = true;
        let options = GeneratorOptions::default();

        let result = generate_type(&typ, &schema_data, Cow::Owned(options));

        let expected_typescript = "string|null";
        let expected_references: Vec<String> = vec![];
        assert_eq!(result.typescript, expected_typescript);
        assert_eq!(result.references, expected_references);
    }
}
