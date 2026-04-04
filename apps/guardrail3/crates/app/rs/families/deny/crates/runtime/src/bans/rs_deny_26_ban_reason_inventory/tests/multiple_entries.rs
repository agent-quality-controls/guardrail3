use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_26_ban_reason_inventory as assertions;

use super::helpers::add_deny_ban_entry;

fn deny_entry_without_reason(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        ("wrappers".to_owned(), toml::Value::Array(Vec::new())),
    ]))
}

#[test]
fn inventories_each_extra_deny_ban_and_counts_them() {
    let deny = add_deny_ban_entry(
        &add_deny_ban_entry(
            "[bans]\ndeny = []\n",
            deny_entry_without_reason("project_only_ban_one"),
        ),
        deny_entry_without_reason("project_only_ban_two"),
    );
    let results = super::helpers::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::info(
                "extra deny ban",
                "`deny.toml` adds deny ban `project_only_ban_one` beyond the managed baseline.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "extra deny ban",
                "`deny.toml` adds deny ban `project_only_ban_two` beyond the managed baseline.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "extra deny ban count",
                "`deny.toml` has 2 deny bans beyond the managed baseline.",
                "deny.toml",
                true,
            ),
        ],
    );
}
