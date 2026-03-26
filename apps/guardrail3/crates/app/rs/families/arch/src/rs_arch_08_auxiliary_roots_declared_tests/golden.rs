use super::super::super::test_support::{check_results, entry, info_results, tree};

#[test]
fn no_auxiliary_info_results_when_no_auxiliary_roots_exist() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "apps/backend/Cargo.toml",
            "[workspace]\nmembers = []\nresolver = \"2\"\n",
        )],
    ));

    assert!(
        info_results(&results, "RS-ARCH-08").is_empty(),
        "unexpected auxiliary info results: {results:#?}"
    );
}
