use crate::extra_method_ban::check;
use g3rs_clippy_config_checks_assertions::extra_method_ban as assertions;
use test_support::input_from_raw;

#[test]
fn inventories_extra_method_ban() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-methods = [{ path = \"example::extra\", reason = \"project-specific\" }]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "extra method ban",
            "Additional method ban `example::extra` beyond baseline.",
            "clippy.toml",
            true,
        )],
    );
}

#[test]
fn inventories_clean_state_when_no_extra_method_bans_exist() {
    let input = input_from_raw("clippy.toml", "disallowed-methods = []\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "no extra method bans",
            "No additional method bans beyond the managed baseline.",
            "clippy.toml",
            true,
        )],
    );
}
