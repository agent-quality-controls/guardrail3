use crate::domain::modules::deny::build_deny_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{
    copy_fixture, run_family, set_license_confidence_threshold, write_file,
};

#[test]
fn local_weaker_confidence_threshold_only_warns_for_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_license_confidence_threshold(
            &build_deny_toml("service", "", "", ""),
            toml::Value::Float(0.7),
        ),
    );

    let results = run_family(tmp.path());
    let license_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-15")
        .collect::<Vec<_>>();

    assert_eq!(license_results.len(), 1, "{license_results:#?}");
    let result = license_results[0];
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "confidence-threshold weaker than baseline");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` sets `confidence-threshold = 0.7`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
}
