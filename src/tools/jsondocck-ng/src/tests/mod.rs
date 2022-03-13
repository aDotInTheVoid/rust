use crate::Tests;

mod ng_example;

mod r#type;

pub(super) fn add_tests(prefix: String, tests: &mut Tests) {
    let prefix = format!("{prefix}::");

    r#type::add_tests(format!("{prefix}type"), tests);

    tests.add(format!("{prefix}ng_example"), ng_example::test);
}
