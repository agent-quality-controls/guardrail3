use std::io::Write;

use super::super::{from_path, parse};
use super::super::ClippyToml;

pub(super) fn parse_fixture(input: &str) -> ClippyToml {
    parse(input).expect("should parse valid clippy.toml")
}

pub(super) fn parse_from_tempfile(input: &str) -> ClippyToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("clippy config should be written");
    from_path(file.path()).expect("file should parse")
}
