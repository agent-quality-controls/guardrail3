use std::os::unix::fs::PermissionsExt;

use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_06_leaf_valid as assertions;

#[test]
#[cfg(unix)]
fn unreadable_valid_leaf_currently_degrades_into_missing_cargo_toml() {
    let tmp = copy_fixture();
    let leaf = tmp.path().join("apps/devctl/crates/domain/types");

    let mut perms = std::fs::metadata(&leaf).expect("metadata").permissions();
    perms.set_mode(0o000);
    std::fs::set_permissions(&leaf, perms).expect("chmod 000");

    let results = super::run_family(tmp.path());

    let mut restore = std::fs::metadata(&leaf).expect("metadata").permissions();
    restore.set_mode(0o755);
    std::fs::set_permissions(&leaf, restore).expect("restore perms");

    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/crates/domain/types"),
            file_contains: None,
            title_contains: Some(&["missing Cargo.toml"]),
            message_contains: None,
        }],
    );
}
