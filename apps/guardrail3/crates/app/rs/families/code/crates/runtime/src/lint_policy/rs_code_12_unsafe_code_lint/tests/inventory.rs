use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_12_unsafe_code_lint::assert_inventories_workspace_forbid_lints;
use test_support::write_file;

#[test]
fn distinguishes_deny_and_forbid_workspace_lint_levels_across_real_manifests() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/Cargo.toml";
    let devctl_rel = "apps/devctl/Cargo.toml";
    let worker_rel = "apps/worker/Cargo.toml";

    let backend_content = test_support::read_file(root, backend_rel);
    let worker_content = test_support::read_file(root, worker_rel);

    write_file(
        root,
        backend_rel,
        &backend_content.replace("unsafe_code = \"forbid\"", "unsafe_code = \"deny\""),
    );
    write_file(
        root,
        worker_rel,
        &worker_content.replace("unsafe_code = \"forbid\"", "unsafe_code = \"forbid\""),
    );

    assert_inventories_workspace_forbid_lints(
        &run_family(root),
        backend_rel,
        devctl_rel,
        worker_rel,
    );
}

#[test]
fn inventories_table_shaped_workspace_lint_levels() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/Cargo.toml";
    let worker_rel = "apps/worker/Cargo.toml";

    let backend_content = test_support::read_file(root, backend_rel);
    let worker_content = test_support::read_file(root, worker_rel);

    write_file(
        root,
        backend_rel,
        &backend_content.replace(
            "unsafe_code = \"forbid\"",
            "unsafe_code = { level = \"deny\" }",
        ),
    );
    write_file(
        root,
        worker_rel,
        &worker_content.replace(
            "unsafe_code = \"forbid\"",
            "unsafe_code = { level = \"forbid\", priority = 0 }",
        ),
    );

    assert_inventories_workspace_forbid_lints(
        &run_family(root),
        backend_rel,
        "apps/devctl/Cargo.toml",
        worker_rel,
    );
}
