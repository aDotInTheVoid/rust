// This test checks that the core library of Rust can be compiled without enabling
// support for formatting and parsing floating-point numbers.

extern crate run_make_support;

use run_make_support::rustc;
use std::path::PathBuf;

fn main() {
    rustc()
        .edition("2021")
        .arg("-Dwarnings")
        .crate_type("rlib")
        .input("../../../library/core/src/lib.rs")
        .cfg("no_fp_fmt_parse")
        .run();
}
