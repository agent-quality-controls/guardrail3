use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::super::check;

#[test]
fn warns_on_unknown_top_level_and_core_section_keys() {
    let config = config_facts(&canonical_deny_toml_service().replace(
        "[graph]\n",
        "extra-root = true\n[graph]\nextra-flag = true\n",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    let actual = results
        .iter()
        .map(|result| (result.title.clone(), result.message.clone()))
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        (
            "unknown graph key".to_owned(),
            "`deny.toml` uses unknown `[graph].extra-flag`.".to_owned(),
        ),
        (
            "unknown top-level deny key".to_owned(),
            "`deny.toml` uses unknown top-level key `extra-root`.".to_owned(),
        ),
    ]);

    assert_eq!(actual, expected);
    assert!(results.iter().all(|result| {
        result.id == "RS-DENY-28"
            && result.severity == Severity::Warn
            && result.file.as_deref() == Some("deny.toml")
    }));
}
