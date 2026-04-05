#![allow(
    clippy::expect_used,
    reason = "parser tests use panic-based helpers for concise proof fixtures"
)]

use std::io::Write;

use crate::{CargoToml, from_path, parse};

pub(super) fn parse_fixture(input: &str) -> CargoToml {
    parse(input).expect("Cargo.toml fixture should parse")
}

pub(super) fn parse_from_tempfile(input: &str) -> CargoToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("Cargo.toml fixture should be written");
    let path = file.path().to_path_buf();
    from_path(&path).expect("Cargo.toml file should parse")
}
