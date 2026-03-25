use std::collections::BTreeMap;

use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, remove_deny_ban,
};
use super::super::check;

#[test]
fn errors_for_each_missing_canonical_service_ban() {
    let config = config_facts(&remove_deny_ban(
        &remove_deny_ban(&canonical_deny_toml_service(), "actix-web"),
        "lazy_static",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    let actual = results
        .iter()
        .map(|result| {
            (
                result.message.clone(),
                (
                    format!("{:?}", result.severity),
                    result.title.clone(),
                    result.file.clone(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let expected = BTreeMap::from([
        (
            "`deny.toml` is missing deny ban `actix-web`.".to_owned(),
            (
                format!("{:?}", Severity::Error),
                "missing canonical ban".to_owned(),
                Some("deny.toml".to_owned()),
            ),
        ),
        (
            "`deny.toml` is missing deny ban `lazy_static`.".to_owned(),
            (
                format!("{:?}", Severity::Error),
                "missing canonical ban".to_owned(),
                Some("deny.toml".to_owned()),
            ),
        ),
    ]);

    assert_eq!(actual, expected);
}
