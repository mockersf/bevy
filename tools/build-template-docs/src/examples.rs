use std::{cmp::Ordering, collections::HashMap};

use serde::Serialize;
use tera::Context;
use toml::Value;

use crate::TemplatedDoc;

#[derive(Default)]
pub struct Examples {
    examples: Vec<Example>,
}

impl TemplatedDoc for Examples {
    fn source_template(&self) -> &'static str {
        "EXAMPLE_README.md.tpl"
    }
    fn destination(&self) -> &'static str {
        "examples/README.md"
    }

    fn parse_from_cargo(&mut self, panic_on_missing: bool) {
        let manifest_file = std::fs::read_to_string("Cargo.toml").unwrap();
        let manifest: HashMap<String, Value> = toml::from_str(&manifest_file).unwrap();
        let metadatas = manifest
            .get("package")
            .unwrap()
            .get("metadata")
            .as_ref()
            .unwrap()["example"]
            .clone();

        self.examples = manifest["example"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|val| {
                let technical_name = val["name"].as_str().unwrap().to_string();
                if panic_on_missing && metadatas.get(&technical_name).is_none() {
                    panic!("Missing metadata for example {technical_name}");
                }

                if metadatas
                    .get(&technical_name)
                    .and_then(|metadata| metadata.get("hidden"))
                    .and_then(|hidden| hidden.as_bool())
                    .and_then(|hidden| hidden.then_some(()))
                    .is_some()
                {
                    return None;
                }

                metadatas.get(&technical_name).map(|metadata| Example {
                    technical_name,
                    path: val["path"].as_str().unwrap().to_string(),
                    name: metadata["name"].as_str().unwrap().to_string(),
                    description: metadata["description"].as_str().unwrap().to_string(),
                    category: metadata["category"].as_str().unwrap().to_string(),
                    wasm: metadata["wasm"].as_bool().unwrap(),
                })
            })
            .collect();
    }

    fn build_context(&mut self) -> Context {
        let categories = parse_categories();
        let examples_by_category: HashMap<String, Category> = self
            .examples
            .drain(..)
            .fold(HashMap::<String, Vec<Example>>::new(), |mut v, ex| {
                v.entry(ex.category.clone()).or_default().push(ex);
                v
            })
            .into_iter()
            .map(|(key, mut examples)| {
                examples.sort();
                let description = categories.get(&key).cloned();
                (
                    key,
                    Category {
                        description,
                        examples,
                    },
                )
            })
            .collect();

        let mut context = Context::new();
        context.insert("all_examples", &examples_by_category);
        context
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct Category {
    description: Option<String>,
    examples: Vec<Example>,
}

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
struct Example {
    technical_name: String,
    path: String,
    name: String,
    description: String,
    category: String,
    wasm: bool,
}

impl Ord for Example {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.category.cmp(&other.category) {
            Ordering::Equal => self.name.cmp(&other.name),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Example {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_categories() -> HashMap<String, String> {
    let manifest_file = std::fs::read_to_string("Cargo.toml").unwrap();
    let manifest: HashMap<String, Value> = toml::from_str(&manifest_file).unwrap();
    manifest
        .get("package")
        .unwrap()
        .get("metadata")
        .as_ref()
        .unwrap()["category"]
        .clone()
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            (
                v.get("name").unwrap().as_str().unwrap().to_string(),
                v.get("description").unwrap().as_str().unwrap().to_string(),
            )
        })
        .collect()
}
