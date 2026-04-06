use std::io::Write;

use super::super::{from_path, parse};
use crate::CliffToml;

pub(super) fn parse_fixture(input: &str) -> CliffToml {
    parse(input).expect("should parse valid cliff.toml")
}

pub(super) fn parse_from_tempfile(input: &str) -> CliffToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("cliff config should be written");
    from_path(file.path()).expect("file should parse")
}
