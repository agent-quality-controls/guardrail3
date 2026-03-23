use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::config_facts;
use super::super::check;

fn ignore_toml(ignore: &str) -> String {
    format!("[advisories]\nignore = {ignore}\n")
}

#[test]
fn warns_for_malformed_missing_reason_and_non_string_reason_ignore_entries() {
    let config = config_facts(&ignore_toml(
        r#"[{ reason = "good enough reason text" }, { id = "RUSTSEC-2026-0001" }, { id = "RUSTSEC-2026-0002", reason = 7 }]"#,
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
            "advisory ignore missing reason".to_owned(),
            "`deny.toml` ignores advisory `RUSTSEC-2026-0001` without a `reason`.".to_owned(),
        ),
        (
            "advisory ignore reason must be a string".to_owned(),
            "`deny.toml` has `[advisories].ignore` entry `RUSTSEC-2026-0002` with a non-string `reason`.".to_owned(),
        ),
        (
            "malformed advisory ignore entry".to_owned(),
            "`deny.toml` has an `[advisories].ignore` entry without a valid advisory id.".to_owned(),
        ),
    ]);

    assert_eq!(actual, expected);
    assert!(results.iter().all(|result| {
        result.id == "RS-DENY-24"
            && result.severity == Severity::Warn
            && !result.inventory
            && result.file.as_deref() == Some("deny.toml")
    }));
}
