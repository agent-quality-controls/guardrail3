#![allow(
    clippy::panic,
    clippy::expect_used,
    reason = "test-only helpers: panic/expect surface the failing fixture loud-and-fast"
)]

use std::io::Write;

use super::super::{Error, Guardrail3TsToml, from_path, parse};

pub(super) fn parse_fixture(input: &str) -> Guardrail3TsToml {
    parse(input).unwrap_or_else(|err| panic!("valid guardrail3-ts.toml should parse: {err}"))
}

pub(super) fn parse_from_tempfile(input: &str) -> Guardrail3TsToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("guardrail3-ts.toml should be written");
    from_path(file.path()).expect("file should parse")
}

pub(super) fn parse_error(input: &str) -> Error {
    match parse(input) {
        Ok(_) => panic!("input should fail to parse"),
        Err(err) => err,
    }
}

pub(super) fn from_path_missing() -> Error {
    let dir = tempfile::tempdir().expect("tempdir should be created");
    let path = dir.path().join("does-not-exist.toml");
    from_path(path).expect_err("missing file should error")
}
