use super::super::check;
use super::super::test_support::{canonical_clippy_toml, root_workspace_tree};

#[test]
fn inventories_extra_type_bans() {
    let clippy = canonical_clippy_toml().replace(
        "disallowed-types = [\n",
        "disallowed-types = [\n    { path = \"std::sync::Arc\", reason = \"good enough reason text\" },\n",
    );
    let results = check(&root_workspace_tree(clippy));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-07" && r.inventory && r.message.contains("std::sync::Arc")));
}
