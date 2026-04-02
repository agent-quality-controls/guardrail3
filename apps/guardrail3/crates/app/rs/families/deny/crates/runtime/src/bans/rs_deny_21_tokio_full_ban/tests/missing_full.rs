use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_21_tokio_full_ban as assertions;

use super::super::expected_tokio_allowed_features_for_test;
use super::super::{build_fixture_deny_toml, set_feature_entries};

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
fn warns_when_tokio_full_is_not_banned() {
    let expected_allow = expected_tokio_allowed_features_for_test()
        .into_iter()
        .collect::<Vec<_>>();
    let allow_refs = expected_allow
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let deny = set_feature_entries(
        &build_fixture_deny_toml("service"),
        vec![tokio_entry(&[], &allow_refs)],
    );
    let results = super::super::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "tokio full feature not banned",
            "`deny.toml` must ban `tokio` feature `full` under `[[bans.features]]`.",
            "deny.toml",
            false,
        )],
    );
}
