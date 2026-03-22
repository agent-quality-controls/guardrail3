use crate::domain::report::Severity;

use super::super::deny_support::{expected_tokio_allowed_features, join_set};
use super::super::inputs::ConfigDenyInput;
use super::super::test_support::config_facts;
use super::check;

fn tokio_feature_toml(deny_features: &[&str], allow_features: &[&str]) -> String {
    let deny = deny_features
        .iter()
        .map(|feature| format!("\"{feature}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let allow = allow_features
        .iter()
        .map(|feature| format!("\"{feature}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[bans]\n\n[[bans.features]]\nname = \"tokio\"\ndeny = [{deny}]\nallow = [{allow}]\n")
}

#[test]
fn warns_when_tokio_full_is_not_banned() {
    let expected_allow = expected_tokio_allowed_features()
        .into_iter()
        .collect::<Vec<_>>();
    let expected_allow_refs = expected_allow
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let config = config_facts(&tokio_feature_toml(&[], &expected_allow_refs));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-21");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "tokio full feature not banned");
    assert_eq!(
        result.message,
        "`deny.toml` must ban `tokio` feature `full` under `[[bans.features]]`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}

#[test]
fn warns_when_tokio_allow_list_drifts() {
    let config = config_facts(&tokio_feature_toml(&["full"], &["rt-multi-thread"]));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-21");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "tokio allowed features changed");
    assert_eq!(
        result.message,
        format!(
            "`deny.toml` must keep `tokio` allowed features `{}`.",
            join_set(&expected_tokio_allowed_features())
        )
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
