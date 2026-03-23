use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::super::check;

#[test]
fn warns_on_unknown_nested_skip_ignore_exception_and_feature_keys() {
    let deny = canonical_deny_toml_service()
        .replace(
            "skip = []",
            "skip = [{ crate = \"serde@1.0.0\", reason = \"good enough reason text\", extra = true }]",
        )
        .replace(
            "ignore = []",
            "ignore = [{ id = \"RUSTSEC-2026-0001\", reason = \"good enough reason text\", extra = true }]",
        )
        .replace(
            "[licenses.private]\nignore = true",
            "[licenses.private]\nignore = true\n\n[[licenses.exceptions]]\nname = \"ring\"\nallow = [\"ISC\"]\nextra = true",
        )
        .replace(
            "[[bans.features]]\nname = \"tokio\"",
            "[[bans.features]]\nname = \"tokio\"\nextra = true",
        );
    let config = config_facts(&deny);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    let actual = results
        .iter()
        .map(|result| (result.title.clone(), result.message.clone()))
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        (
            "unknown advisories.ignore key".to_owned(),
            "`deny.toml` uses unknown `[[advisories.ignore]].extra` at index 0.".to_owned(),
        ),
        (
            "unknown bans.skip key".to_owned(),
            "`deny.toml` uses unknown `[[bans.skip]].extra` at index 0.".to_owned(),
        ),
        (
            "unknown feature-ban key".to_owned(),
            "`deny.toml` uses unknown `[[bans.features]].extra`.".to_owned(),
        ),
        (
            "unknown licenses.exceptions key".to_owned(),
            "`deny.toml` uses unknown `[[licenses.exceptions]].extra` at index 0.".to_owned(),
        ),
    ]);

    assert_eq!(actual, expected);
    assert!(results.iter().all(|result| {
        result.id == "RS-DENY-28"
            && result.severity == Severity::Warn
            && result.file.as_deref() == Some("deny.toml")
    }));
}
