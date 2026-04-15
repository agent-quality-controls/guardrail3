use crate::rs_clippy_config_13_ban_reason_quality::check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_13_ban_reason_quality as assertions;
use test_support::input_from_raw;

#[test]
fn errors_on_plain_string_ban_entries_without_reason() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-methods = [\"serde_json::from_str\"]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "ban entry missing reason",
            "`serde_json::from_str` in `disallowed-methods` must use table format with a `reason` field.",
            "clippy.toml",
            false,
        )],
    );
}

#[test]
fn inventories_reasoned_table_ban_entries() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-methods = [{ path = \"serde_json::from_str\", reason = \"Use typed boundary parsing\" }]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        assertions::findings(&results),
        vec![assertions::info(
            "ban entries use reasoned table format",
            "All managed ban entries use table format with a `reason` field.",
            "clippy.toml",
            true,
        )]
    );
}

#[test]
fn errors_on_malformed_macro_ban_sections() {
    let input = input_from_raw("clippy.toml", "disallowed-macros = [1]\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "ban section malformed",
            "`disallowed-macros[0]` must be a string or table, found integer.",
            "clippy.toml",
            false,
        )],
    );
}
