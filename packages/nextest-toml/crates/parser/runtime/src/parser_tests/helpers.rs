use std::io::Write;

use super::super::{from_path, parse};
use nextest_toml_types::NextestToml;

pub(super) fn parse_fixture(input: &str) -> NextestToml {
    parse(input).expect("should parse valid nextest.toml")
}

pub(super) fn parse_from_tempfile(input: &str) -> NextestToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("nextest config should be written");
    from_path(file.path()).expect("file should parse")
}
