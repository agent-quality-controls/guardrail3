use crate::rs_clippy_config_21_config_parseable::check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_21_config_parseable as assertions;
use test_support::input_from_raw;

#[test]
fn inventories_parseable_clippy_toml() {
    let input = input_from_raw("clippy.toml", "max-struct-bools = 3\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "clippy.toml parseable",
            "`clippy.toml` parsed successfully.",
            "clippy.toml",
            true,
        )],
    );
}

#[test]
fn errors_on_typed_parse_failures() {
    let input = input_from_raw("clippy.toml", "disallowed-methods = [{ path = 7 }]\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_parse_error_contains(&results, "Failed to parse `clippy.toml`");
}

#[test]
fn errors_on_raw_toml_parse_failures() {
    let input = input_from_raw("clippy.toml", "not = [valid");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_parse_error_contains(&results, "Failed to parse `clippy.toml`");
}
