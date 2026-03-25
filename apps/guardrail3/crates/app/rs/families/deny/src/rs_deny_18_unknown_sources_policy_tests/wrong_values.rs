use std::collections::BTreeMap;

use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_source_policy,
};
use super::super::check;

#[test]
fn errors_for_each_weakened_unknown_source_policy_key() {
    let deny = set_source_policy(
        &set_source_policy(&canonical_deny_toml_service(), "unknown-registry", "allow"),
        "unknown-git",
        "warn",
    );
    let config = config_facts(&deny);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    let actual = results
        .iter()
        .map(|result| {
            (
                result.title.clone(),
                (
                    format!("{:?}", result.severity),
                    result.message.clone(),
                    result.file.clone(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let expected = BTreeMap::from([
        (
            "sources `unknown-git` has wrong value".to_owned(),
            (
                format!("{:?}", Severity::Error),
                "`deny.toml` must set `[sources].unknown-git = \"deny\"`.".to_owned(),
                Some("deny.toml".to_owned()),
            ),
        ),
        (
            "sources `unknown-registry` has wrong value".to_owned(),
            (
                format!("{:?}", Severity::Error),
                "`deny.toml` must set `[sources].unknown-registry = \"deny\"`.".to_owned(),
                Some("deny.toml".to_owned()),
            ),
        ),
    ]);

    assert_eq!(actual, expected);
}
