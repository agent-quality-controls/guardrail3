use std::io::Write;

use super::super::{from_path, parse};
use crate::Guardrail3RsToml;

pub(super) fn parse_fixture(input: &str) -> Guardrail3RsToml {
    parse(input).expect("should parse valid guardrail3-rs.toml")
}

pub(super) fn parse_fixture_file(path: &str) -> Guardrail3RsToml {
    let full_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("parser_tests")
        .join("fixtures")
        .join(path);
    from_path(full_path).expect("fixture file should parse")
}

pub(super) fn parse_from_tempfile(input: &str) -> Guardrail3RsToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("guardrail3-rs.toml should be written");
    from_path(file.path()).expect("file should parse")
}
