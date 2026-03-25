use super::super::super::test_support::{check_results, entry, error_results, tree};

#[test]
fn misplaced_other_roots_do_not_count_as_ambiguous_classification() {
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &[])),
            ("tools", entry(&["worker"], &[])),
            ("tools/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[("tools/worker/Cargo.toml", "[package]\nname = \"worker\"\n")],
    ));

    assert!(
        error_results(&results, "RS-ARCH-01").is_empty(),
        "other roots should stay owned by RS-ARCH-02, not RS-ARCH-01: {results:#?}"
    );
}
