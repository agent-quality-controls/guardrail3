use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_26_ban_reason_inventory as assertions;

use super::super::add_deny_ban_entry;

#[test]
fn treats_whitespace_only_reason_as_missing_and_counts_it() {
    let deny = add_deny_ban_entry(
        "[bans]\ndeny = []\n",
        toml::Value::Table(toml::map::Map::from_iter([
            ("name".to_owned(), toml::Value::String("json5".to_owned())),
            ("wrappers".to_owned(), toml::Value::Array(Vec::new())),
            ("reason".to_owned(), toml::Value::String("   ".to_owned())),
        ])),
    );
    let results = super::super::run_check(&deny);

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "ban entry missing reason",
                "`deny.toml` ban entry `json5` has no `reason`.",
                "deny.toml",
                false,
            ),
            assertions::warn_no_file(
                "ban entry count",
                "`deny.toml` has 1 deny ban entries (0 documented, 1 missing reasons, 0 weak reasons).",
                false,
            ),
        ],
    );
}
