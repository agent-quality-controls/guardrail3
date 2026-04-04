use crate::{CargoToml, from_path, parse};

pub(super) fn parse_fixture(input: &str) -> CargoToml {
    parse(input).expect("Cargo.toml fixture should parse")
}

pub(super) fn parse_from_tempfile(input: &str) -> CargoToml {
    let temp = tempfile::tempdir().expect("create temp dir for Cargo.toml parser test");
    let path = temp.path().join("Cargo.toml");
    std::fs::write(&path, input).expect("write Cargo.toml fixture");
    from_path(&path).expect("Cargo.toml file should parse")
}
