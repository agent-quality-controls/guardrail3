use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn errors_when_canonical_ban_is_missing() {
    let config = config_facts(
        &canonical_deny_toml_service().replace("{ name = \"actix-web\", wrappers = [] },\n", ""),
    );
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-09"
            && result.severity == Severity::Error
            && result.title == "missing canonical ban"
            && result.message == "`deny.toml` is missing deny ban `actix-web`."
            && result.file.as_deref() == Some("deny.toml")
    }));
}

#[test]
fn library_profile_requires_library_io_bans() {
    let mut config = config_facts(
        &canonical_deny_toml_service()
        .replace("{ name = \"axum\", wrappers = [] },\n", "")
        .replace("{ name = \"tokio\", wrappers = [] },\n", ""),
    );
    config.profile_name = Some("library".to_owned());
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-09" && result.message == "`deny.toml` is missing deny ban `axum`."
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-09" && result.message == "`deny.toml` is missing deny ban `tokio`."
    }));
}
