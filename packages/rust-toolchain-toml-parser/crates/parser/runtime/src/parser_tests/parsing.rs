use helpers::{parse_fixture, parse_from_tempfile};
use rust_toolchain_toml_parser_runtime_assertions::parser as assertions;

use super::helpers;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_toolchain_absent(&cfg);
    assertions::assert_top_level_extra_empty(&cfg);
}

#[test]
fn toolchain_section_parses_common_fields() {
    let cfg = parse_fixture(
        r#"
[toolchain]
channel = "stable"
components = ["clippy", "rustfmt"]
targets = ["wasm32-unknown-unknown"]
profile = "minimal"
"#,
    );

    assertions::assert_toolchain_fields(
        &cfg,
        Some("stable"),
        None,
        &["clippy", "rustfmt"],
        &["wasm32-unknown-unknown"],
        Some("minimal"),
    );
    assertions::assert_toolchain_extra_empty(&cfg);
}

#[test]
fn missing_arrays_default_to_empty() {
    let cfg = parse_fixture(
        r#"
[toolchain]
channel = "1.85.0"
"#,
    );

    assertions::assert_toolchain_fields(&cfg, Some("1.85.0"), None, &[], &[], None);
}

#[test]
fn toolchain_section_parses_path_form() {
    let cfg = parse_fixture(
        r#"
[toolchain]
path = "/opt/rust/toolchains/local"
"#,
    );

    assertions::assert_toolchain_fields(
        &cfg,
        None,
        Some("/opt/rust/toolchains/local"),
        &[],
        &[],
        None,
    );
}

#[test]
fn unknown_top_level_keys_are_captured() {
    let cfg = parse_fixture(
        r#"
schema-version = 1

[toolchain]
channel = "stable"
"#,
    );

    assertions::assert_top_level_integer_extra(&cfg, "schema-version", 1);
}

#[test]
fn unknown_toolchain_keys_are_captured() {
    let cfg = parse_fixture(
        r#"
[toolchain]
channel = "stable"
custom-key = "kept"
"#,
    );

    assertions::assert_toolchain_string_extra(&cfg, "custom-key", "kept");
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(
        r#"
[toolchain]
channel = "stable"
components = ["clippy"]
"#,
    );

    assertions::assert_toolchain_fields(&cfg, Some("stable"), None, &["clippy"], &[], None);
    assertions::assert_toolchain_extra_empty(&cfg);
}
