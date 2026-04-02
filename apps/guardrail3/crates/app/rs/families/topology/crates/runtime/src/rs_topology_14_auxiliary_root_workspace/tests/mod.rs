use test_support::{entry, tree};

#[test]
fn auxiliary_top_level_root_must_be_workspace() {
    let tree = tree(
        &[
            ("", entry(&["tools"], &[])),
            ("tools", entry(&["helper"], &[])),
            ("tools/helper", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "tools/helper/Cargo.toml",
            "[package]\nname = \"helper\"\n\n[package.metadata.guardrail3]\ntopology_role = \"auxiliary\"\n",
        )],
    );

    let results = crate::check_test_tree(&tree);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-TOPOLOGY-14")
        .expect("expected RS-TOPOLOGY-14 result");

    assert_eq!(result.file(), Some("tools/helper/Cargo.toml"));
}
