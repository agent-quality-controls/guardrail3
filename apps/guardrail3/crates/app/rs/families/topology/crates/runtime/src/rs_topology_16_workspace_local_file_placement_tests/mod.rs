use test_support::{entry, tree};

mod nested_cargo_sidecar;
mod nested_clippy;
mod nested_deny;
mod nested_fmt;
mod nested_toolchain;
mod outside_workspace_guardrail;
mod repo_root_toolchain;
mod workspace_root_toolchain;

fn check_results(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}

#[test]
fn misplaced_workspace_local_file_is_reported_by_topology() {
    let tree = tree(
        &[
            ("", entry(&["apps", "tools"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
            ("tools", entry(&["helper"], &[])),
            ("tools/helper", entry(&[], &["clippy.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = []\nresolver = \"2\"\n",
            ),
            ("tools/helper/clippy.toml", "msrv = \"1.85\"\n"),
        ],
    );

    let results = crate::check_test_tree(&tree);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-TOPOLOGY-16")
        .expect("expected RS-TOPOLOGY-16 result");

    assert_eq!(result.file(), Some("tools/helper/clippy.toml"));
    assert!(
        result
            .message()
            .contains("outside every legal workspace root for `clippy`"),
        "unexpected message: {}",
        result.message()
    );
}
