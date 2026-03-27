use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_item_level_and_file_backed_module_near_misses() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let module_decl_rel = "apps/devctl/crates/adapters/inbound/cli/src/lib.rs";
    let file_backed_parent_rel = "apps/backend/crates/ports/outbound/events/src/lib.rs";
    let item_rel = "apps/worker/crates/ports/outbound/queue/src/lib.rs";

    let module_decl_content =
        std::fs::read_to_string(root.join(module_decl_rel)).expect("read module decl file");
    let file_backed_parent_content =
        std::fs::read_to_string(root.join(file_backed_parent_rel)).expect("read parent file");
    let item_content = std::fs::read_to_string(root.join(item_rel)).expect("read item file");

    write_file(
        root,
        module_decl_rel,
        &format!("{module_decl_content}\n#[allow(clippy::unwrap_used)]\nmod file_backed_allow;\n"),
    );
    write_file(
        root,
        "apps/devctl/crates/adapters/inbound/cli/src/file_backed_allow.rs",
        "pub fn helper() {}\n",
    );
    write_file(
        root,
        file_backed_parent_rel,
        &format!("{file_backed_parent_content}\nmod file_backed_child;\n"),
    );
    write_file(
        root,
        "apps/backend/crates/ports/outbound/events/src/file_backed_child.rs",
        "#![allow(clippy::panic)]\npub fn helper() {}\n",
    );
    write_file(
        root,
        item_rel,
        &format!("{item_content}\n#[allow(clippy::expect_used)]\npub fn item_level_probe() {{}}\n"),
    );

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-01"),
        BTreeSet::from([
            "apps/backend/crates/ports/outbound/events/src/file_backed_child.rs".to_owned()
        ])
    );
    assert!(!results.iter().any(|result| {
        result.id == "RS-CODE-01" && result.file.as_deref() == Some(file_backed_parent_rel)
    }));
}
