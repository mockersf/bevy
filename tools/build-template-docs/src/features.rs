use std::{cmp::Ordering, collections::HashMap};

use serde::Serialize;
use tera::Context;
use toml::Value;

use crate::TemplatedDoc;

#[derive(Default)]
pub struct Features {
    features: Vec<Feature>,
}

impl TemplatedDoc for Features {
    fn source_template(&self) -> &'static str {
        "cargo_features.md.tpl"
    }
    fn destination(&self) -> &'static str {
        "docs/cargo_features.md"
    }

    fn parse_from_cargo(&mut self, panic_on_missing: bool) {
        let manifest_file = std::fs::read_to_string("Cargo.toml").unwrap();
        let manifest: HashMap<String, Value> = toml::from_str(&manifest_file).unwrap();
        let metadatas = manifest
            .get("package")
            .unwrap()
            .get("metadata")
            .as_ref()
            .unwrap()["feature"]
            .clone();

        self.features = manifest["features"]
            .as_table()
            .unwrap()
            .iter()
            .flat_map(|(key, val)| {
                eprintln!("feature: {} -> {}", key, val);
                if panic_on_missing && metadatas.get(&key).is_none() {
                    panic!("Missing metadata for feature {key}");
                }

                if metadatas
                    .get(&key)
                    .and_then(|metadata| metadata.get("hidden"))
                    .and_then(|hidden| hidden.as_bool())
                    .and_then(|hidden| hidden.then_some(()))
                    .is_some()
                {
                    return None;
                }

                metadatas.get(&key).map(|metadata| Feature {
                    name: key.clone(),
                    description: metadata["description"].as_str().unwrap().to_string(),
                    dependencies: val
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|val| val.to_string())
                        .collect(),
                })
            })
            .collect();
        dbg!(&self.features);
    }

    fn build_context(&mut self) -> Context {
        let mut context = Context::new();
        context
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
struct Feature {
    name: String,
    description: String,
    dependencies: Vec<String>,
}
