use g3rs_deny_config_checks_assertions::rs_deny_config_26_extra_deny_bans_inventory as assertions;

use crate::test_support::{canonical_bans_toml, run};

#[test]
fn inventories_extra_deny_bans_beyond_the_managed_baseline() {
    let mut parsed =
        toml::from_str::<toml::Value>(&canonical_bans_toml("service")).expect("valid deny fixture");
    let deny_entries = parsed
        .get_mut("bans")
        .and_then(toml::Value::as_table_mut)
        .and_then(|bans| bans.get_mut("deny"))
        .and_then(toml::Value::as_array_mut)
        .expect("expected bans.deny array");
    deny_entries.push(toml::Value::String("custom-crate".to_owned()));
    let deny_toml = toml::to_string(&parsed).expect("serialize deny");

    let results = run(
        &deny_toml,
        Some("service"),
        true,
        crate::rs_deny_config_26_extra_deny_bans_inventory::check,
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::info(
                "extra deny ban",
                "`deny.toml` adds deny ban `custom-crate` beyond the managed baseline.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "extra deny ban count",
                "`deny.toml` has 1 deny bans beyond the managed baseline.",
                "deny.toml",
                true,
            ),
        ],
    );
}
