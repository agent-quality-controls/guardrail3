use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn warns_on_duplicate_deny_entries() {
    let deny = config_facts(&canonical_deny_toml_service().replace(
        "{ name = \"json5\", wrappers = [] },\n",
        "{ name = \"json5\", wrappers = [] },\n    { name = \"json5\", wrappers = [] },\n",
    ));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-27");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "duplicate deny entry");
    assert_eq!(
        result.message,
        "`deny.toml` has duplicate deny entry `json5`."
    );
}

#[test]
fn warns_on_duplicate_ignore_entries() {
    let deny = config_facts(&canonical_deny_toml_service().replace(
        "ignore = []",
        "ignore = [\"RUSTSEC-2020-0001\", \"RUSTSEC-2020-0001\"]",
    ));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-27");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "duplicate advisory ignore entry");
    assert_eq!(
        result.message,
        "`deny.toml` has duplicate advisory ignore `RUSTSEC-2020-0001`."
    );
}

#[test]
fn warns_on_duplicate_feature_entries() {
    let deny = config_facts(&canonical_deny_toml_service().replace(
        "reason = \"good enough reason text\"\n",
        "reason = \"good enough reason text\"\n\n[[bans.features]]\nname = \"tokio\"\ndeny = [\"full\"]\nallow = [\"fs\"]\nreason = \"good enough reason text\"\n",
    ));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-27");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "duplicate feature-ban entry");
    assert_eq!(
        result.message,
        "`deny.toml` has duplicate `[[bans.features]]` for `tokio`."
    );
}
