use openapiv3::ReferenceOr;

use openapiv3_to_typescript::parse_schema;

use crate::helpers::fetch_oas_yaml;

#[test]
fn xero_accounting_works() {
    let oas = fetch_oas_yaml(
        "https://raw.githubusercontent.com/XeroAPI/Xero-OpenAPI/master/xero_accounting.yaml",
    );

    let components = oas.components.unwrap();
    for (name, reference_or_schema) in components.schemas {
        println!("SCHEMA: {}", name);
        match reference_or_schema {
            ReferenceOr::Reference { .. } => todo!(),
            ReferenceOr::Item(schema) => {
                let result = parse_schema(schema).unwrap();
                dbg!(result);
            }
        }
    }
}
