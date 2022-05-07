use toml::Value;

fn main() {
    let manifest = std::fs::read_to_string("Cargo.toml").unwrap();
    let manifest: Value = toml::from_str(&manifest).unwrap();
    let metadata = manifest["metadata"].clone()["example"].clone();

    // put modules in bevy_examples/lib.rs for each category
    // put examples as struct in the correct module
    // put description as documentation
    // put a link to a page with the example as wasm available at the root

    for example in manifest["example"].as_array().unwrap() {
        if let Some(metadata) = metadata.get(example["name"].as_str().unwrap()) {
            println!("{:?}", example);
            println!("-- {:?}", metadata);
        } else {
            continue;
        }
    }
}
