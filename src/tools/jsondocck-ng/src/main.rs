#![feature(assert_matches)]

// This currently doesn't rerun tests when test code changes
// See https://rust-lang.zulipchat.com/#narrow/stream/182449-t-compiler.2Fhelp/topic/rustbuild.2Fcompiletest.20question.3A.20Many.20or.20One.20test.20crate
// Untill someone answers and I fix
// rg --files-with-matches use-jsondocck-ng src/test/rustdoc-json/ | xargs touch
//  ./x.py test src/test/rustdoc-json/

mod config;
mod tcrate;
mod tests;

use std::{assert_matches::assert_matches, collections::HashMap};

use rustdoc_json_types::Crate;

type Test = fn(tcrate::TCrate);

#[derive(Debug, Default)]
struct Tests {
    tests: HashMap<String, Test>,
}

impl Tests {
    fn add(&mut self, name: String, test: Test) {
        assert_matches!(self.tests.insert(name, test), None, "No duplicate tests");
    }
}

fn main() -> anyhow::Result<()> {
    let conf = config::Config::new();
    let mut tests = Tests::default();
    tests::add_tests("".to_owned(), &mut tests);

    dbg!(&tests, &conf);

    let json_s = std::fs::read_to_string(&conf.json_path)?;
    let krate: Crate = serde_json::from_str(&json_s)?;
    let tc = tcrate::TCrate { krate };

    let test = &tests.tests[&*conf.test_name];

    test(tc);

    panic!("Exiting with failure to show output");

    Ok(())
}
