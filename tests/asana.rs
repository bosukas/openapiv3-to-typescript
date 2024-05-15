use openapiv3::ReferenceOr;

use openapiv3_to_typescript::parse_schema;

use crate::helpers::fetch_oas_yaml;

#[test]
fn asana_works() {
    let oas = fetch_oas_yaml(
        "https://raw.githubusercontent.com/Asana/openapi/master/defs/asana_oas.yaml",
    );

    let components = oas.components.unwrap();
    for (name, reference_or_schema) in components.schemas {
        println!("SCHEMA: {}", name);
        match reference_or_schema {
            ReferenceOr::Reference { reference } => {
                dbg!(reference);
            }
            ReferenceOr::Item(schema) => {
                let result = parse_schema(schema).unwrap();
                println!("{:?}", result.references);
                println!("{}", result.typescript);
            }
        }
    }
}
