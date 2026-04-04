use std::io::Write;

use super::super::{from_path, parse};
use crate::RustfmtToml;

pub(super) fn parse_fixture(input: &str) -> RustfmtToml {
    parse(input).expect("should parse valid rustfmt.toml")
}

pub(super) fn parse_from_tempfile(input: &str) -> RustfmtToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("rustfmt config should be written");
    from_path(file.path()).expect("file should parse")
}
