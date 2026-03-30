use guardrail3_app_rs_family_deny_assertions::rs_deny_26_ban_reason_inventory as assertions;

use super::super::add_deny_ban_entry;

fn deny_entry_without_reason(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        ("wrappers".to_owned(), toml::Value::Array(Vec::new())),
    ]))
}

#[test]
fn inventories_each_ban_entry_without_reason() {
    let deny = add_deny_ban_entry(
        &add_deny_ban_entry(
            "[bans]\ndeny = []\n",
            deny_entry_without_reason("lazy_static"),
        ),
        deny_entry_without_reason("json5"),
    );
    let results = super::super::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "ban entry missing reason",
                "`deny.toml` ban entry `json5` has no `reason`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "ban entry missing reason",
                "`deny.toml` ban entry `lazy_static` has no `reason`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
