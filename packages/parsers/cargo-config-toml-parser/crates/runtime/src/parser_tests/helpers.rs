use std::io::Write;

use super::super::{from_path, parse};

pub(super) fn parse_fixture(
    input: &str,
) -> cargo_config_toml_parser_types::cargo_config_toml::CargoConfigToml {
    parse(input).expect("should parse valid cargo config")
}

pub(super) fn parse_from_tempfile(
    input: &str,
) -> cargo_config_toml_parser_types::cargo_config_toml::CargoConfigToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("cargo config should be written");
    from_path(file.path()).expect("file should parse")
}
