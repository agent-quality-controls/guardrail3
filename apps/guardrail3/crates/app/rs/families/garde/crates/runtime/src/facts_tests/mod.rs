use super::collect;
use crate::test_fixtures::family_route;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn root_workspace_uses_packages_garde_policy_when_packages_config_owns_root() {
    let root = temp_root("rs-garde-facts-root-packages");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["packages"],
                    &["Cargo.toml", "guardrail3.toml", "clippy.toml"],
                ),
            ),
            ("packages", dir_entry(&["shared-types"], &[])),
            (
                "packages/shared-types",
                dir_entry(&["src"], &["Cargo.toml"]),
            ),
            ("packages/shared-types/src", dir_entry(&[], &["lib.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                "[workspace]\nmembers = [\"packages/shared-types\"]\n[workspace.dependencies]\ngarde = \"0.22\"\n",
            ),
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.packages]\ntype = \"library\"\n[rust.packages.checks]\ngarde = false\n",
            ),
            (
                "clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            (
                "packages/shared-types/Cargo.toml",
                "[package]\nname = \"shared-types\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = { workspace = true }\n",
            ),
            ("packages/shared-types/src/lib.rs", "pub struct Shared;\n"),
        ],
        root.clone(),
    );

    let facts = collect(&tree, &family_route(&tree, None));

    assert!(
        facts.roots.iter().all(|root| root.rel_dir != ""),
        "root workspace should be gated off by [rust.packages.checks].garde = false: {:#?}",
        facts.roots
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}
