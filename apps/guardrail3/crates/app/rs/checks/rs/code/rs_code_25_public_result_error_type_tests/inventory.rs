use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_weak_public_result_error_types_in_library_profile_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";

    let package_content =
        std::fs::read_to_string(root.join(package_rel)).expect("read package source");
    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");

    write_file(
        root,
        package_rel,
        &format!(
            "{package_content}\n\npub fn parse_shared_slug() -> Result<TenantSlug, String> {{\n    Err(\"missing tenant\".to_owned())\n}}\n"
        ),
    );
    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\n\npub fn planner_boundary_probe() -> Result<Task, Box<dyn std::error::Error>> {{\n    Err(Box::<std::io::Error>::new(std::io::Error::other(\"planner probe\")))\n}}\n"
        ),
    );

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-25"),
        BTreeSet::from([package_rel.to_owned(), backend_rel.to_owned()])
    );
}
