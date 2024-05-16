use std::borrow::Cow;

use openapiv3::{OpenAPI, Operation, Parameter, ParameterSchemaOrContent, PathItem};

use crate::{GeneratorOptions, PartialGeneration};
use crate::schema::generate_schema;

pub fn parse_path<'a>(
    path_item: &'a PathItem,
    operation: &'a Operation,
    openapi: &OpenAPI,
) -> PartialGeneration<'a> {
    let mut typescript = String::new();
    typescript.push('{');

    for param in &path_item.parameters {
        let resolved_param = param.resolve(openapi).unwrap();
        dbg!(&resolved_param.kind);
        let generation = resolve_param(resolved_param, openapi);
        let is_required = match resolved_param.required {
            true => "",
            false => "?",
        };
        typescript.push_str(&format!(
            "{}{}:{};",
            resolved_param.name, is_required, generation.typescript
        ))
    }
    for param in &operation.parameters {
        let resolved_param = param.resolve(openapi).unwrap();
        dbg!(&resolved_param.kind);
        let generation = resolve_param(resolved_param, openapi);
        let is_required = match resolved_param.required {
            true => "",
            false => "?",
        };
        typescript.push_str(&format!(
            "{}{}:{};",
            resolved_param.name, is_required, generation.typescript
        ))
    }

    typescript.push('}');

    PartialGeneration {
        typescript: Cow::Owned(typescript),
        references: vec![],
        read_only: false,
    }
}

pub fn resolve_param<'a>(param: &'a Parameter, openapi: &'a OpenAPI) -> PartialGeneration<'a> {
    match &param.format {
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
