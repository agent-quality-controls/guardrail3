use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn warns_on_unknown_top_level_and_section_keys() {
    let config = config_facts(
        &canonical_deny_toml_service().replace("[graph]\n", "extra-root = true\n[graph]\nextra-flag = true\n"),
    );
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-28"
            && result.severity == Severity::Warn
            && result.title == "unknown top-level deny key"
            && result.message == "`deny.toml` uses unknown top-level key `extra-root`."
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-28"
            && result.severity == Severity::Warn
            && result.title == "unknown graph key"
            && result.message == "`deny.toml` uses unknown `[graph].extra-flag`."
    }));
}

#[test]
fn warns_on_unknown_nested_skip_ignore_and_exception_keys() {
    let deny = canonical_deny_toml_service()
        .replace("skip = []", "skip = [{ crate = \"serde@1.0.0\", reason = \"good enough reason text\", extra = true }]")
        .replace("ignore = []", "ignore = [{ id = \"RUSTSEC-2026-0001\", reason = \"good enough reason text\", extra = true }]")
        .replace(
            "[licenses.private]\nignore = true",
            "[licenses.private]\nignore = true\n\n[[licenses.exceptions]]\nname = \"ring\"\nallow = [\"ISC\"]\nextra = true",
        );
    let config = config_facts(&deny);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-28"
            && result.severity == Severity::Warn
            && result.title == "unknown bans.skip key"
            && result.message == "`deny.toml` uses unknown `[[bans.skip]].extra` at index 0."
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-28"
            && result.severity == Severity::Warn
            && result.title == "unknown advisories.ignore key"
            && result.message
                == "`deny.toml` uses unknown `[[advisories.ignore]].extra` at index 0."
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-28"
            && result.severity == Severity::Warn
            && result.title == "unknown licenses.exceptions key"
            && result.message
                == "`deny.toml` uses unknown `[[licenses.exceptions]].extra` at index 0."
    }));
}
