use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo::rerun-if-changed=openapi.yaml");

    let spec_path = PathBuf::from("openapi.yaml");
    let spec_content = fs::read_to_string(&spec_path).expect("Failed to read openapi.yaml");

    // Parse YAML to JSON Value first, then convert to OpenAPI
    let spec_json: serde_json::Value =
        serde_yaml::from_str(&spec_content).expect("Failed to parse openapi.yaml");
    let spec_string = serde_json::to_string(&spec_json).expect("Failed to serialize to JSON");
    let spec: openapiv3::OpenAPI =
        serde_json::from_str(&spec_string).expect("Failed to parse as OpenAPI");

    let mut generator = progenitor::Generator::default();
    let tokens = generator
        .generate_tokens(&spec)
        .expect("Failed to generate client code");

    let ast = syn::parse2(tokens).expect("Failed to parse generated tokens");
    let content = prettyplease::unparse(&ast);

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("codegen.rs");
    fs::write(&out_path, content).expect("Failed to write generated code");
}
