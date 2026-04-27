use g3rs_deny_config_checks_assertions::extra_deny_bans_inventory as assertions;

use test_support::run;

use super::helpers;

#[test]
fn inventories_extra_deny_bans_beyond_the_managed_baseline() {
    let mut parsed = toml::from_str::<toml::Value>(&helpers::service_canonical_bans_toml())
        .expect("valid deny fixture");
    let deny_entries = parsed
        .get_mut("bans")
        .and_then(toml::Value::as_table_mut)
        .and_then(|bans| bans.get_mut("deny"))
        .and_then(toml::Value::as_array_mut)
        .expect("expected bans.deny array");
    deny_entries.push(toml::Value::String("custom-crate".to_owned()));
    let deny_toml =
        toml::to_string(&parsed).expect("serialize deny fixture after mutating parsed TOML");

    let results = run(
        &deny_toml,
        Some(guardrail3_rs_toml_parser::types::RustProfile::Service),
        true,
        crate::extra_deny_bans_inventory::check,
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
