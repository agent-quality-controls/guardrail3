use crate::domain::modules::deny::build_deny_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, set_feature_entries, write_file};

fn tokio_entry(deny: &[&str], allow: &[&str]) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String("tokio".to_owned())),
        (
            "deny".to_owned(),
            toml::Value::Array(
                deny.iter()
                    .map(|v| toml::Value::String((*v).to_owned()))
                    .collect(),
            ),
        ),
        (
            "allow".to_owned(),
            toml::Value::Array(
                allow
                    .iter()
                    .map(|v| toml::Value::String((*v).to_owned()))
                    .collect(),
            ),
        ),
        (
            "reason".to_owned(),
            toml::Value::String("good enough reason text".to_owned()),
        ),
    ]))
}

#[test]
fn local_tokio_drift_only_warns_for_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_feature_entries(
            &build_deny_toml("service", "", "", ""),
            vec![tokio_entry(&["full"], &["rt-multi-thread"])],
        ),
    );

    let results = super::super::super::test_support::run_family(tmp.path());
    let tokio_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-21")
        .collect::<Vec<_>>();

    assert_eq!(tokio_results.len(), 1, "{tokio_results:#?}");
    let result = tokio_results[0];
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "tokio allowed features changed");
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
    assert!(!result.inventory);
}
