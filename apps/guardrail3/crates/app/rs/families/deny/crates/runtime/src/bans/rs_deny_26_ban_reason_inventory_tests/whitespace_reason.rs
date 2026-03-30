use guardrail3_app_rs_family_deny_assertions::rs_deny_26_ban_reason_inventory as assertions;

use super::super::add_deny_ban_entry;

#[test]
fn inventories_whitespace_only_reason_as_missing() {
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
        &[assertions::info(
            "ban entry missing reason",
            "`deny.toml` ban entry `json5` has no `reason`.",
            "deny.toml",
            true,
        )],
    );
}
