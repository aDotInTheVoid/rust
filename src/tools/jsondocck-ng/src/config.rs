use std::path::PathBuf;

use getopts::Options;

#[derive(Debug)]
pub struct Config {
    pub json_path: PathBuf,
    pub test_name: String,
}

impl Config {
    pub fn new() -> Self {
        let args = std::env::args().collect::<Vec<_>>();
        let mut opts = Options::new();

        opts.reqopt("", "json-path", "Path to generated json", "PATH").reqopt(
            "",
            "test-name",
            "The name of the test being used",
            "name",
        );

        let matches = opts.parse(&args[1..]).unwrap();
        Self {
            json_path: PathBuf::from(matches.opt_str("json-path").unwrap()),
            test_name: matches.opt_str("test-name").unwrap(),
        }
    }
}
