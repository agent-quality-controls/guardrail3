use std::io::Write;

use super::super::{from_path, parse};
use crate::ReleasePlzToml;

pub(super) fn parse_fixture(input: &str) -> ReleasePlzToml {
    parse(input).expect("should parse valid release-plz.toml")
}

pub(super) fn parse_from_tempfile(input: &str) -> ReleasePlzToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("release-plz config should be written");
    from_path(file.path()).expect("file should parse")
}
