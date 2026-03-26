use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_03_inbound_outbound as assertions;
use crate::test_support::{copy_fixture, remove_dir};
const FIXTURE: crate::test_support::HexarchFixture = crate::test_support::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

#[test]
fn directional_child_symlink_to_valid_directory_hits_missing_for_that_container() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/adapters/inbound");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/adapters/outbound"),
        tmp.path().join("apps/devctl/crates/adapters/inbound"),
    )
    .expect("symlink");

    let results = assertions::run_family(tmp.path());
    let devctl_rule_03: Vec<_> = assertions::errors_by_id(&results, "RS-HEXARCH-03")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates/adapters"))
        .collect();

    assert_eq!(devctl_rule_03.len(), 1, "{devctl_rule_03:#?}");
    assert!(
        devctl_rule_03[0].title.contains("missing")
            && devctl_rule_03[0].title.contains("crates/adapters/inbound/"),
        "{devctl_rule_03:#?}"
    );
}

#[test]
fn nested_directional_child_symlink_to_valid_directory_hits_only_the_nested_container() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{}/ports/outbound", inner_hex()));
    std::os::unix::fs::symlink(
        tmp.path().join(format!("{}/ports/inbound", inner_hex())),
        tmp.path().join(format!("{}/ports/outbound", inner_hex())),
    )
    .expect("symlink");

    let results = assertions::run_family(tmp.path());
    let nested_rule_03: Vec<_> = assertions::errors_by_id(&results, "RS-HEXARCH-03")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some(&format!("{}/ports", inner_hex())))
        .collect();

    assert_eq!(nested_rule_03.len(), 1, "{nested_rule_03:#?}");
    assert!(
        nested_rule_03[0]
            .title
            .contains("adapters/inbound/mcp/crates/ports/outbound/"),
        "{nested_rule_03:#?}"
    );
}

#[test]
fn unexpected_directional_child_symlink_is_still_reported_as_unexpected() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/adapters/outbound"),
        tmp.path().join("apps/devctl/crates/adapters/shared"),
    )
    .expect("symlink");

    let results = assertions::run_family(tmp.path());
    let devctl_rule_03: Vec<_> = assertions::errors_by_id(&results, "RS-HEXARCH-03")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates/adapters/shared"))
        .collect();

    assert_eq!(devctl_rule_03.len(), 1, "{devctl_rule_03:#?}");
    assert!(
        devctl_rule_03[0].title.contains("unexpected")
            && devctl_rule_03[0].title.contains("crates/adapters/shared/"),
        "{devctl_rule_03:#?}"
    );
}
