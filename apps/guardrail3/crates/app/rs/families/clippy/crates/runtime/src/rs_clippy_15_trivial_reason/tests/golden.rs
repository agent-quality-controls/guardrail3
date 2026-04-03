use guardrail3_app_rs_family_clippy_assertions::rs_clippy_15_trivial_reason as assertions;
use test_support::{build_fixture_clippy_toml, root_workspace_tree};

use super::helpers::run_for_tests;

#[test]
fn warns_when_ban_reasons_are_substantive() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CLIPPY-15"
                && result.file() == Some("clippy.toml")
                && result.title() == "ban entry uses documented escape hatch"
        }),
        "expected documented clippy ban warnings: {results:#?}"
    );
    assertions::assert_count_summary(
        &results,
        "`clippy.toml` has 96 clippy ban entries (96 documented, 0 missing reasons, 0 weak reasons).",
    );
}
