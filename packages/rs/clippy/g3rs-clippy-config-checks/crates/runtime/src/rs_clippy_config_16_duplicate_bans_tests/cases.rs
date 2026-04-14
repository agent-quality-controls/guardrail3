use crate::rs_clippy_config_16_duplicate_bans::check;
use crate::test_support::{findings, input_from_raw};

#[test]
fn warns_on_duplicate_ban_entries() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-macros = [\"std::println\", \"std::println\"]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "duplicate ban entry" && finding.message.contains("std::println")
    }));
}

#[test]
fn inventories_duplicate_free_ban_sections() {
    let input = input_from_raw("clippy.toml", "disallowed-macros = [\"std::println\"]\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(
        findings(&results).iter().any(|finding| {
            finding.title == "ban entries are duplicate-free" && finding.inventory
        })
    );
}

#[test]
fn warns_on_duplicate_method_bans() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-methods = [\"serde_json::from_str\", \"serde_json::from_str\"]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "duplicate ban entry" && finding.message.contains("serde_json::from_str")
    }));
}
