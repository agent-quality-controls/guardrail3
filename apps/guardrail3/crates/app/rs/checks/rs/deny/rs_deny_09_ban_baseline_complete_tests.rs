use super::super::check;
use super::super::test_support::{
    canonical_deny_toml_service, library_profile_tree, root_tree_with_deny,
};

#[test]
fn errors_when_canonical_ban_is_missing() {
    let deny = canonical_deny_toml_service().replace("{ name = \"actix-web\", wrappers = [] },\n", "");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-09" && r.message.contains("actix-web")));
}

#[test]
fn library_profile_requires_library_io_bans() {
    let deny = canonical_deny_toml_service()
        .replace("{ name = \"axum\", wrappers = [] },\n", "")
        .replace("{ name = \"tokio\", wrappers = [] },\n", "");
    let results = check(&library_profile_tree(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-09" && r.message.contains("axum")));
    assert!(results.iter().any(|r| r.id == "RS-DENY-09" && r.message.contains("tokio")));
}
