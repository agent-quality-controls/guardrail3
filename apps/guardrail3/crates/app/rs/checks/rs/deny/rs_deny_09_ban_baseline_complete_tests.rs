use crate::domain::report::Severity;

use super::super::deny_support::expected_bans;
use super::super::inputs::ConfigDenyInput;
use super::super::test_support::config_facts;
use super::check;

fn bans_toml(profile: Option<&str>) -> String {
    let entries = expected_bans(profile)
        .into_iter()
        .map(|(name, expectation)| {
            let wrappers = expectation
                .wrappers
                .into_iter()
                .map(|wrapper| format!("\"{wrapper}\""))
                .collect::<Vec<_>>()
                .join(", ");
            format!("    {{ name = \"{name}\", wrappers = [{wrappers}] }},")
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("[bans]\ndeny = [\n{entries}\n]\n")
}

#[test]
fn errors_when_canonical_ban_is_missing() {
    let config = config_facts(
        &bans_toml(None).replace("    { name = \"actix-web\", wrappers = [] },\n", ""),
    );
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-09");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "missing canonical ban");
    assert_eq!(
        result.message,
        "`deny.toml` is missing deny ban `actix-web`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}

#[test]
fn library_profile_requires_library_io_bans() {
    let mut config = config_facts(
        &bans_toml(Some("library"))
            .replace("    { name = \"axum\", wrappers = [] },\n", "")
            .replace("    { name = \"tokio\", wrappers = [] },\n", ""),
    );
    config.profile_name = Some("library".to_owned());
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-09"
            && result.severity == Severity::Error
            && result.title == "missing canonical ban"
            && result.message == "`deny.toml` is missing deny ban `axum`."
            && result.file.as_deref() == Some("deny.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-09"
            && result.severity == Severity::Error
            && result.title == "missing canonical ban"
            && result.message == "`deny.toml` is missing deny ban `tokio`."
            && result.file.as_deref() == Some("deny.toml")
    }));
}
