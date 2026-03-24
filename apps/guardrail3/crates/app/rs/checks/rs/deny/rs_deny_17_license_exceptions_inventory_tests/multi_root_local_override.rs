use crate::domain::modules::deny::build_deny_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{
    copy_fixture, run_family, set_license_exceptions, write_file,
};

#[test]
fn local_license_exception_only_inventories_for_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_license_exceptions(
            &build_deny_toml("service", "", "", ""),
            vec![toml::Value::Table(toml::map::Map::from_iter([
                (
                    "crate".to_owned(),
                    toml::Value::String("windows-sys".to_owned()),
                ),
                (
                    "allow".to_owned(),
                    toml::Value::Array(vec![toml::Value::String("Zlib".to_owned())]),
                ),
            ]))],
        ),
    );

    let results = run_family(tmp.path());
    let license_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-17")
        .collect::<Vec<_>>();

    assert_eq!(license_results.len(), 1, "{license_results:#?}");
    let result = license_results[0];
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "license exception entry");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` has license exception for `windows-sys`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
    assert!(result.inventory);
}
