use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_26_ban_reason_inventory as assertions;

use super::helpers::add_deny_ban_entry;

#[test]
fn ignores_reason_text_for_canonical_deny_entries() {
    let deny = add_deny_ban_entry(
        "[bans]\ndeny = []\n",
        toml::Value::Table(toml::map::Map::from_iter([
            ("name".to_owned(), toml::Value::String("json5".to_owned())),
            ("wrappers".to_owned(), toml::Value::Array(Vec::new())),
            ("reason".to_owned(), toml::Value::String("   ".to_owned())),
        ])),
    );
    let results = super::helpers::run_check(&deny);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "no extra deny bans",
            "`deny.toml` has 0 deny bans beyond the managed baseline.",
            "deny.toml",
            true,
        )],
    );
}
