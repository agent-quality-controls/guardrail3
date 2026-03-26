use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_03_inbound_outbound as assertions;
use crate::test_support::{copy_fixture, remove_dir, write_file};

#[test]
fn missing_parent_directional_container_is_owned_by_rule_02_not_rule_03() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters");

    let results = assertions::run_family(tmp.path());
    let devctl_rule_03: Vec<_> = assertions::errors_by_id(&results, "RS-HEXARCH-03")
        .into_iter()
        .filter(|error| {
            error
                .file
                .as_deref()
                .is_some_and(|file| file.starts_with("apps/devctl/crates/adapters"))
        })
        .collect();

    assert!(devctl_rule_03.is_empty(), "{devctl_rule_03:#?}");
}

#[test]
fn parent_directional_container_replaced_with_file_is_owned_by_rule_02_not_rule_03() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/ports");
    write_file(tmp.path(), "apps/devctl/crates/ports", "not a directory");

    let results = assertions::run_family(tmp.path());
    let devctl_rule_03: Vec<_> = assertions::errors_by_id(&results, "RS-HEXARCH-03")
        .into_iter()
        .filter(|error| {
            error
                .file
                .as_deref()
                .is_some_and(|file| file.starts_with("apps/devctl/crates/ports"))
        })
        .collect();

    assert!(devctl_rule_03.is_empty(), "{devctl_rule_03:#?}");
}
