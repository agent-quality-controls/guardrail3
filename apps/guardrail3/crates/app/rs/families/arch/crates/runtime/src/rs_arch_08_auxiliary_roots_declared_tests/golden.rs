use guardrail3_app_rs_family_arch_assertions::rs_arch_08_auxiliary_roots_declared as assertions;
#[allow(unused_imports)]
use test_support::{APP_WORKSPACE_CARGO, PACKAGE_CARGO, entry, tree, tree_at};

#[test]
fn no_auxiliary_info_results_when_no_auxiliary_roots_exist() {
    let results = assertions::check_results(&tree(
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
        assertions::info_results(&results, "RS-ARCH-08").is_empty(),
        "unexpected auxiliary info results: {results:#?}"
    );
}
