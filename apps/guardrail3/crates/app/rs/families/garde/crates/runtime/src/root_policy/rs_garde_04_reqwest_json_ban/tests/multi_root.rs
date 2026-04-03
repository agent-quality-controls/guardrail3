use guardrail3_app_rs_family_garde_assertions::rs_garde_04_reqwest_json_ban as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn local_missing_reqwest_ban_only_warns_for_owned_root() {
    let root = temp_root("multi-garde-04");
    let root_clippy = super::helpers::canonical_clippy_toml();
    let local_clippy = super::helpers::remove_clippy_ban_path(
        &super::helpers::canonical_clippy_toml(),
        "disallowed-methods",
        "reqwest::Response::json",
    );
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["root", "lib"], &[])),
            ("apps/root", dir_entry(&[], &["Cargo.toml", "clippy.toml"])),
            ("apps/lib", dir_entry(&[], &["Cargo.toml", "clippy.toml"])),
        ],
        vec![
            (
                "apps/root/Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"root\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = \"0.1\"\n",
            ),
            ("apps/root/clippy.toml", root_clippy.as_str()),
            (
                "apps/lib/Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"vendored\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = \"0.1\"\n",
            ),
            ("apps/lib/clippy.toml", local_clippy.as_str()),
        ],
        root.clone(),
    );
    let results = super::helpers::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(
        findings.len(),
        2,
        "unexpected RS-GARDE-04 findings: {findings:#?}"
    );
    assertions::assert_inventory(
        &results,
        "apps/root/clippy.toml",
        "`reqwest::Response::json` is banned in the covering clippy configuration.",
    );
    assertions::assert_missing(
        &results,
        "apps/lib/clippy.toml",
        "Missing `reqwest::Response::json` from `disallowed-methods`. Add it to `disallowed-methods` in clippy.toml.",
    );
    std::fs::remove_dir_all(&root).expect("remove temp root");
}
