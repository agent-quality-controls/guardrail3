use crate::duplicate_bans::check;
use g3rs_clippy_config_checks_assertions::duplicate_bans as assertions;
use test_support::input_from_raw;

#[test]
fn warns_on_duplicate_ban_entries() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-macros = [\"std::println\", \"std::println\"]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "duplicate ban entry",
            "`std::println` appears 2 times in `disallowed-macros`. Remove the duplicate entries.",
            "clippy.toml",
            false,
        )],
    );
}

#[test]
fn inventories_duplicate_free_ban_sections() {
    let input = input_from_raw("clippy.toml", "disallowed-macros = [\"std::println\"]\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "ban entries are duplicate-free",
            "Managed ban sections contain no duplicate paths.",
            "clippy.toml",
            true,
        )],
    );
}

#[test]
fn warns_on_duplicate_method_bans() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-methods = [\"serde_json::from_str\", \"serde_json::from_str\"]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "duplicate ban entry",
            "`serde_json::from_str` appears 2 times in `disallowed-methods`. Remove the duplicate entries.",
            "clippy.toml",
            false,
        )],
    );
}
