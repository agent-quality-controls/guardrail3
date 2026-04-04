use std::io::Write;

use super::super::{from_path, parse};
use deny_toml_types::DenyToml;

pub(super) fn parse_fixture(input: &str) -> DenyToml {
    parse(input).expect("should parse valid deny.toml")
}

pub(super) fn parse_from_tempfile(input: &str) -> DenyToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("deny config should be written");
    from_path(file.path()).expect("file should parse")
}
