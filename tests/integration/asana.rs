use openapiv3_to_typescript::{Generator, GeneratorOptions};

use crate::helpers::fetch_oas_yaml;

#[test]
fn asana_works() {
    let oas = fetch_oas_yaml(
        "https://raw.githubusercontent.com/Asana/openapi/master/defs/asana_oas.yaml",
    );
    let options = GeneratorOptions {
        skip_read_only: true,
    };
    let generator = Generator::new(oas, options);

    for (name, schema) in &generator.openapi().schemas {
        let parsed_schema = schema.resolve(generator.openapi());
        let partial_generation = generator.generate_schema(parsed_schema, None);
        println!("type {}={};", name.as_str(), partial_generation.typescript)
    }
}
