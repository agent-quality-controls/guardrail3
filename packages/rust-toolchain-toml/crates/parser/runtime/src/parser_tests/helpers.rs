use std::io::Write;

use super::super::{from_path, parse};
use crate::RustToolchainToml;

pub(super) fn parse_fixture(input: &str) -> RustToolchainToml {
    parse(input).expect("should parse valid rust-toolchain.toml")
}

pub(super) fn parse_from_tempfile(input: &str) -> RustToolchainToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("toolchain file should be written");
    from_path(file.path()).expect("file should parse")
}
