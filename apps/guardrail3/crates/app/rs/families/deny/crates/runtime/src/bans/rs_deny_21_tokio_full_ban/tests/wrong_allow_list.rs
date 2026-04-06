use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_config_16_tokio_full_ban as assertions;

use super::helpers::{build_fixture_deny_toml, set_feature_entries};
use super::helpers::{expected_tokio_allowed_features_for_test, join_set_for_test};

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
fn warns_when_tokio_allow_list_drifts() {
    let deny = set_feature_entries(
        &build_fixture_deny_toml("service"),
        vec![tokio_entry(&["full"], &["rt-multi-thread"])],
    );
    let results = super::helpers::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "tokio allowed features changed",
            &format!(
                "`deny.toml` must keep `tokio` allowed features `{}`.",
                join_set_for_test(&expected_tokio_allowed_features_for_test())
            ),
            "deny.toml",
            false,
        )],
    );
}
