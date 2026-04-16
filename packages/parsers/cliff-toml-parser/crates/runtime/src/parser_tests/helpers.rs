use std::io::Write;

use super::super::{from_path, parse};

pub(super) fn parse_fixture(input: &str) -> cliff_toml_parser_types::cliff_toml::CliffToml {
    parse(input).expect("should parse valid cliff.toml")
}

pub(super) fn parse_from_tempfile(input: &str) -> cliff_toml_parser_types::cliff_toml::CliffToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("cliff config should be written");
    from_path(file.path()).expect("file should parse")
}
