use std::io::Write;

use super::super::{from_path, parse};
use crate::MutantsToml;

pub(super) fn parse_fixture(input: &str) -> MutantsToml {
    parse(input).expect("should parse valid mutants.toml")
}

pub(super) fn parse_from_tempfile(input: &str) -> MutantsToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("mutants config should be written");
    from_path(file.path()).expect("file should parse")
}
