use rust_toolchain_toml_parser_runtime_assertions::parser as assertions;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = assertions::parse_fixture("");

    assertions::assert_toolchain_absent(&cfg);
    assertions::assert_top_level_extra_empty(&cfg);
}

#[test]
fn toolchain_section_parses_common_fields() {
    let cfg = assertions::parse_fixture(
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
    let cfg = assertions::parse_fixture(
        r#"
[toolchain]
channel = "1.85.0"
"#,
    );

    assertions::assert_toolchain_fields(&cfg, Some("1.85.0"), None, &[], &[], None);
}

#[test]
fn toolchain_section_parses_path_form() {
    let cfg = assertions::parse_fixture(
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
    let cfg = assertions::parse_fixture(
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
    let cfg = assertions::parse_fixture(
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
    use std::io::Write as _;

    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(
        br#"
[toolchain]
channel = "stable"
components = ["clippy"]
"#,
    )
    .expect("toolchain file should be written");
    let cfg = crate::parser::from_path(file.path()).expect("file should parse");

    assertions::assert_toolchain_fields(&cfg, Some("stable"), None, &["clippy"], &[], None);
    assertions::assert_toolchain_extra_empty(&cfg);
}

#[test]
fn path_and_channel_cannot_be_combined() {
    let err = assertions::parse_error(
        r#"
[toolchain]
channel = "stable"
path = "/opt/rust/toolchains/local"
"#,
    )
    .expect_err("path + channel should be rejected");

    let message = err.to_string();
    assert!(
        message.contains("cannot specify both channel"),
        "expected path + channel error, got: {message}",
    );
}

#[test]
fn path_cannot_be_combined_with_other_toolchain_options() {
    let err = assertions::parse_error(
        r#"
[toolchain]
path = "/opt/rust/toolchains/local"
components = ["clippy"]
targets = ["wasm32-unknown-unknown"]
profile = "minimal"
"#,
    )
    .expect_err("path + other toolchain options should be rejected");

    let message = err.to_string();
    assert!(
        message.contains("toolchain options are ignored for path toolchain"),
        "expected path toolchain options error, got: {message}",
    );
}
