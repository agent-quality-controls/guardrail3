use guardrail3_app_rs_family_clippy_assertions::rs_clippy_19_unknown_keys as assertions;
use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn warns_for_managed_key_typos_but_not_unrelated_unknown_keys() {
    let tree = root_workspace_tree(
        "disalowed-methods = []\nallow-print-in-tets = false\ncustom-project-key = true\nmsrv = \"1.85\"\n",
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_messages(
        &results,
        &[
            "Top-level key `allow-print-in-tets` looks like a typo of a guardrail-managed clippy key.",
            "Top-level key `disalowed-methods` looks like a typo of a guardrail-managed clippy key.",
        ],
        "clippy.toml",
    );
}
