use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    copy_fixture, run_family, set_bans_allow_entries, write_file,
};

#[test]
fn local_allow_list_presence_only_errors_for_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_bans_allow_entries(
            &build_deny_toml("service", "", "", ""),
            vec![toml::Value::String("totally-custom-crate".to_owned())],
        ),
    );

    let results = run_family(tmp.path());
    let allow_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-25")
        .collect::<Vec<_>>();

    assert_eq!(allow_results.len(), 1, "{allow_results:#?}");
    let result = allow_results[0];
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "bans allow-list present");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` has non-empty `[bans].allow`: totally-custom-crate."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
    assert!(!result.inventory);
}
