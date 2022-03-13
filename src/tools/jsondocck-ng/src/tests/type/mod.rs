use crate::Tests;

mod generic_default_ng;

pub(super) fn add_tests(prefix: String, tests: &mut Tests) {
    let prefix = format!("{prefix}::");

    tests.add(format!("{prefix}generic_default_ng"), generic_default_ng::test);
}
