use openapiv3::{SchemaData, StringType};

use crate::Parser;
use crate::partial::PartialTypeScript;

pub fn parse_string(_parser: &Parser, string: &StringType, data: &SchemaData) -> PartialTypeScript {
    let ts = match string.enumeration.is_empty() {
        true => "string".to_string(),
        false => string
            .enumeration
            .iter()
            .map(|item| format!("\"{}\"", item))
            .collect::<Vec<String>>()
            .join("|"),
    };
    let ts = match data.nullable {
        true => format!("{}|null", ts),
        false => ts,
    };

    PartialTypeScript {
        ts,
        references: vec![],
    }
}
