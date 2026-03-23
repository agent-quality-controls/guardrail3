use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::config_facts;
use super::super::check;

fn skip_toml(skip: &str) -> String {
    format!("[bans]\nskip = {skip}\n")
}

#[test]
fn warns_for_malformed_missing_reason_and_non_string_reason_entries() {
    let config = config_facts(&skip_toml(
        r#"[{ reason = "good enough reason text" }, { crate = "serde@1.0.0" }, { crate = "regex@1.0.0", reason = 7 }]"#,
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
            "malformed skip entry".to_owned(),
            "`deny.toml` has `[bans.skip]` entry without a valid crate identifier.".to_owned(),
        ),
        (
            "skip entry missing reason".to_owned(),
            "`deny.toml` skips `serde` without a `reason`.".to_owned(),
        ),
        (
            "skip reason must be a string".to_owned(),
            "`deny.toml` has `[bans.skip]` entry `regex` with a non-string `reason`.".to_owned(),
        ),
    ]);

    assert_eq!(actual, expected);
    assert!(results.iter().all(|result| {
        result.id == "RS-DENY-23"
            && result.severity == Severity::Warn
            && !result.inventory
            && result.file.as_deref() == Some("deny.toml")
    }));
}
