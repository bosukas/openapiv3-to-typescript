use std::borrow::Cow;

use openapiv3::{OpenAPI, Operation, Parameter, ParameterSchemaOrContent, PathItem, RefOr};

use crate::{GeneratorOptions, PartialGeneration};
use crate::schema::generate_schema;

pub fn parse_path<'a>(
    path_item: &'a PathItem,
    operation: &'a Operation,
    openapi: &OpenAPI,
) -> PartialGeneration<'a> {
    let mut typescript = String::from('{');

    parse_params(&mut typescript, &path_item.parameters, &openapi);
    parse_params(&mut typescript, &operation.parameters, &openapi);

    typescript.push('}');

    PartialGeneration {
        typescript: Cow::Owned(typescript),
        references: vec![],
        read_only: false,
    }
}

fn parse_params(typescript: &mut String, parameters: &Vec<RefOr<Parameter>>, openapi: &OpenAPI) {
    for parameter in parameters {
        let resolved_parameter = parameter.resolve(openapi).unwrap();
        let generation = parse_param(resolved_parameter, openapi);
        let is_required = match resolved_parameter.required {
            true => "",
            false => "?",
        };
        typescript.push_str(&format!(
            "{}{}:{};",
            resolved_parameter.name, is_required, generation.typescript
        ))
    }
}

fn parse_param<'a>(parameter: &'a Parameter, openapi: &'a OpenAPI) -> PartialGeneration<'a> {
    match &parameter.format {
        ParameterSchemaOrContent::Schema(schema) => {
            let schema = schema.resolve(openapi);
            generate_schema(
                schema,
                Cow::Owned(GeneratorOptions {
                    skip_read_only: true,
                }),
            )
        }
        ParameterSchemaOrContent::Content(_) => unimplemented!(),
    }
}
