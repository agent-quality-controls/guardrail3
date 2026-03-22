use super::super::check;
use super::super::test_support::root_tree_with_deny;

#[test]
fn inventories_ban_entries_without_reason() {
    let deny = r#"
[graph]
all-features = true
no-default-features = false

[bans]
multiple-versions = "deny"
wildcards = "allow"
allow-wildcard-paths = true
highlight = "all"
deny = [
    { name = "regex", wrappers = [] },
]
skip = []

[licenses]
allow = ["MIT"]
confidence-threshold = 0.8

[licenses.private]
ignore = true

[advisories]
unmaintained = "workspace"
yanked = "warn"
ignore = []

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
allow-git = []
"#;
    let results = check(&root_tree_with_deny(deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-26" && r.inventory));
}
