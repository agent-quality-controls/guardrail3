use crate::domain::report::Severity;

use super::super::deny_support::expected_licenses;
use super::super::inputs::ConfigDenyInput;
use super::super::test_support::config_facts;
use super::check;

fn licenses_toml() -> String {
    let allow = expected_licenses()
        .into_iter()
        .map(|license| format!("\"{license}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[licenses]\nallow = [{allow}]\n\n[licenses.private]\nignore = true\n")
}

#[test]
fn errors_when_license_baseline_is_missing() {
    let config = config_facts(&licenses_toml().replace("\"MIT\", ", ""));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-14");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "baseline license missing");
    assert_eq!(
        result.message,
        "`deny.toml` is missing allowed license `MIT`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
