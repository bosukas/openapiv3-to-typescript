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
    for (name, _) in &generator.openapi().paths.paths {
        dbg!(name);
    }
    let r = generator
        .generate_path("createSubtaskForTask".to_string())
        .unwrap();
    println!("{}", r.typescript);
}
