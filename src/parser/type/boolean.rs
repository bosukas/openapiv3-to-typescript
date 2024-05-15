use openapiv3::SchemaData;

use crate::Parser;
use crate::partial::PartialTypeScript;

pub fn parse_boolean(_parser: &Parser, _data: &SchemaData) -> PartialTypeScript {
    PartialTypeScript {
        ts: "boolean".to_string(),
        references: vec![],
    }
}
