use g3rs_topology_file_tree_checks_assertions::{ExpectedRuleResult, assert_rule_results};
use guardrail3_check_types::G3Severity;

use crate::test_support::input;

#[test]
fn escaping_member_path_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\", \"../shared\"]\n",
        Vec::new(),
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-FILETREE-13",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("Workspace `.` uses escaping member path `../shared`"),
            file: Some("Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn absolute_member_path_fires() {
    let input = input(
        "[workspace]\nmembers = [\"/tmp/shared\"]\n",
        Vec::new(),
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-FILETREE-13",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("Workspace `.` uses escaping member path `/tmp/shared`"),
            file: Some("Cargo.toml"),
            inventory: Some(false),
            message: Some("`Cargo.toml` declares member pattern `/tmp/shared`, which points outside the workspace directory. Workspace members must be relative subdirectory paths inside the workspace root, not absolute paths or `..` escapes. Change the pattern to a relative subdirectory path, or move the target crate inside the workspace."),
        }],
    );
}

#[test]
fn normal_member_path_stays_quiet() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\"]\n",
        Vec::new(),
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(&results, "RS-TOPOLOGY-FILETREE-13", &[]);
}
