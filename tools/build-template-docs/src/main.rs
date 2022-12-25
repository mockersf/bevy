use std::fs::File;

use bitflags::bitflags;
use tera::{Context, Tera};

mod examples;
mod features;

bitflags! {
    struct Command: u32 {
        const CHECK_MISSING = 0b00000001;
        const UPDATE = 0b00000010;
    }

    struct Target: u32 {
        const EXAMPLES = 0b00000001;
        const FEATURES = 0b00000010;
    }
}

impl Target {
    fn to_templated_doc(&self) -> Box<dyn TemplatedDoc> {
        if *self == Target::EXAMPLES {
            return Box::new(examples::Examples::default());
        }
        if *self == Target::FEATURES {
            return Box::new(features::Features::default());
        }
        unreachable!()
    }
}

trait TemplatedDoc {
    fn source_template(&self) -> &'static str;
    fn destination(&self) -> &'static str;

    fn parse_from_cargo(&mut self, panic_on_missing: bool);
    fn build_context(&mut self) -> Context;
}

fn main() {
    let what_to_run = match std::env::args().nth(1).as_deref() {
        Some("check-missing") => Command::CHECK_MISSING,
        Some("update") => Command::UPDATE,
        _ => Command::all(),
    };

    let targets = match std::env::args().nth(2).as_deref() {
        Some("examples") => Target::EXAMPLES,
        Some("features") => Target::FEATURES,
        _ => Target::all(),
    };

    for target in [Target::EXAMPLES, Target::FEATURES] {
        if targets.contains(target) {
            let mut templated = target.to_templated_doc();
            templated.parse_from_cargo(what_to_run.contains(Command::CHECK_MISSING));
            let context = templated.build_context();
            Tera::new("tools/templates/*.md.tpl")
                .expect("error parsing template")
                .render_to(
                    templated.source_template(),
                    &context,
                    File::create(templated.destination()).expect("error creating file"),
                )
                .expect("error rendering template");
        }
    }
}
