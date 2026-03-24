use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::super::check;

#[test]
fn warns_on_unknown_bans_and_sources_keys() {
    let deny = canonical_deny_toml_service()
        .replace("[bans]\n", "[bans]\nextra-ban-flag = true\n")
        .replace("[sources]\n", "[sources]\nextra-source-flag = true\n");
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
            "unknown bans key".to_owned(),
            "`deny.toml` uses unknown `[bans].extra-ban-flag`.".to_owned(),
        ),
        (
            "unknown sources key".to_owned(),
            "`deny.toml` uses unknown `[sources].extra-source-flag`.".to_owned(),
        ),
    ]);

    assert_eq!(actual, expected);
    assert!(results.iter().all(|result| {
        result.id == "RS-DENY-28"
            && result.severity == Severity::Warn
            && result.file.as_deref() == Some("deny.toml")
    }));
}
