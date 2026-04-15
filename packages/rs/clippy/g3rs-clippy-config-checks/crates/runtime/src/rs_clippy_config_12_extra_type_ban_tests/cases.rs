use crate::rs_clippy_config_12_extra_type_ban::check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_12_extra_type_ban as assertions;
use test_support::input_from_raw;

#[test]
fn inventories_extra_type_ban() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-types = [{ path = \"example::extra\", reason = \"project-specific\" }]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "extra type ban",
            "Additional type ban `example::extra` beyond baseline.",
            "clippy.toml",
            true,
        )],
    );
}

#[test]
fn inventories_clean_state_when_no_extra_type_bans_exist() {
    let input = input_from_raw("clippy.toml", "disallowed-types = []\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "no extra type bans",
            "No additional type bans beyond the managed baseline.",
            "clippy.toml",
            true,
        )],
    );
}

#[test]
fn reports_malformed_type_sections() {
    let input = input_from_raw("clippy.toml", "disallowed-types = [1]\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "disallowed-types section malformed",
            "`disallowed-types[0]` must be a string or table, found integer.",
            "clippy.toml",
            false,
        )],
    );
}
