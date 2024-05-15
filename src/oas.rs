use openapiv3::{OpenAPI, Schema};

use crate::parse_schema;
use crate::partial::PartialTypeScript;

#[derive(Debug)]
pub struct Parser {
    options: ParserOptions,
    oas: OpenAPI,
}

#[derive(Debug, Default)]
pub struct ParserOptions {
    pub skip_read_only: bool,
}

impl Parser {
    pub fn new(options: ParserOptions, oas: OpenAPI) -> Self {
        Self { options, oas }
    }

    pub fn oas(&self) -> &OpenAPI {
        &self.oas
    }

    pub fn parse_schema(&self, schema: &Schema) -> PartialTypeScript {
        parse_schema(&self, schema)
    }
}
