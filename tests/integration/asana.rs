use openapiv3_to_typescript::{Parser, ParserOptions};

use crate::helpers::fetch_oas_yaml;

#[test]
fn asana_works() {
    let oas = fetch_oas_yaml(
        "https://raw.githubusercontent.com/Asana/openapi/master/defs/asana_oas.yaml",
    );
    let parser_options = ParserOptions {
        skip_read_only: false,
    };
    let parser = Parser::new(parser_options, oas);

    for (name, schema) in &parser.oas().schemas {
        let schema = schema.resolve(parser.oas());
        let partial = parser.parse_schema(schema);
        println!("type {} = {}", name.as_str(), partial.ts,)
    }
}
