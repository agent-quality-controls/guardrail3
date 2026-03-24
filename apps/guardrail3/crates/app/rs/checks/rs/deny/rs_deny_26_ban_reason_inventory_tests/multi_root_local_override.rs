use crate::domain::modules::deny::build_deny_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{
    copy_fixture, remove_deny_ban_reason, run_family, write_file,
};

#[test]
fn local_missing_ban_reason_only_inventories_for_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &remove_deny_ban_reason(&build_deny_toml("service", "", "", ""), "json5"),
    );

    let results = run_family(tmp.path());
    let reason_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-26")
        .collect::<Vec<_>>();

    assert_eq!(reason_results.len(), 1, "{reason_results:#?}");
    let result = reason_results[0];
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "ban entry missing reason");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` ban entry `json5` has no `reason`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
    assert!(result.inventory);
}
