use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_on_unknown_top_level_and_section_keys() {
    let deny = canonical_deny_toml_service().replace("[graph]\n", "extra-root = true\n[graph]\nextra-flag = true\n");
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-28"
            && result.title == "unknown top-level deny key"
            && result.message.contains("extra-root")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-28"
            && result.title == "unknown graph key"
            && result.message.contains("[graph].extra-flag")
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
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-28"
            && result.title == "unknown bans.skip key"
            && result.message.contains("[[bans.skip]].extra")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-28"
            && result.title == "unknown advisories.ignore key"
            && result.message.contains("[[advisories.ignore]].extra")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-28"
            && result.title == "unknown licenses.exceptions key"
            && result.message.contains("[[licenses.exceptions]].extra")
    }));
}
