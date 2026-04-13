use super::check;
use crate::support::expected_required_type_bans;
use crate::test_support::{findings, input_from_raw, parsed_policy};

#[test]
fn reports_missing_baseline_type_ban() {
    let input = input_from_raw("clippy.toml", "disallowed-types = []\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    let findings = findings(&results);
    let missing = findings
        .iter()
        .filter(|finding| finding.title == "missing type ban")
        .collect::<Vec<_>>();

    assert_eq!(
        missing.len(),
        expected_required_type_bans(true).len(),
        "{findings:#?}"
    );
    assert!(
        missing
            .iter()
            .any(|finding| finding.message.contains("std::collections::HashMap"))
    );
}

#[test]
fn reports_library_profile_specific_missing_type_ban() {
    let input = crate::test_support::input_with_raw(
        "clippy.toml",
        "disallowed-types = []\n",
        parsed_policy("guardrail3.toml", Some("library"), true),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "missing type ban" && finding.message.contains("std::sync::Mutex")
    }));
}

#[test]
fn drops_garde_owned_type_bans_when_garde_is_disabled() {
    let input = crate::test_support::input_with_raw(
        "clippy.toml",
        "disallowed-types = []\n",
        parsed_policy("guardrail3.toml", Some("service"), false),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    let findings = findings(&results);
    let missing = findings
        .iter()
        .filter(|finding| finding.title == "missing type ban")
        .collect::<Vec<_>>();
    assert_eq!(
        missing.len(),
        expected_required_type_bans(false).len(),
        "{findings:#?}"
    );
}
