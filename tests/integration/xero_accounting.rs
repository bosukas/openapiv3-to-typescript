use openapiv3_to_typescript::{Parser, ParserOptions};

use crate::helpers::fetch_oas_yaml;

#[test]
fn xero_accounting_works() {
    let oas = fetch_oas_yaml(
        "https://raw.githubusercontent.com/XeroAPI/Xero-OpenAPI/master/xero_accounting.yaml",
    );
    let parser_options = ParserOptions {
        skip_read_only: true,
    };
    let parser = Parser::new(parser_options, oas);

    for (name, schema) in &parser.oas().schemas {
        let schema = schema.resolve(parser.oas());
        let partial = parser.parse_schema(schema);
        println!("type {} = {}", name.as_str(), partial.ts,)
    }
}
