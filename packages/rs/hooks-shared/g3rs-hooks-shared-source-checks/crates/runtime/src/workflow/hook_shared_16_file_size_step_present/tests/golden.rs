use g3rs_hooks_shared_source_checks_assertions::workflow::hook_shared_16_file_size_step_present as assertions;

use super::run_case;

#[test]
fn warns_when_file_size_only_appears_in_comment() {
    let results = run_case("# git cat-file -s :$file\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("file-size check step missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_assignment_runs_git_cat_file_size() {
    let results = run_case(r#"file_size=$(git cat-file -s ":$file")"#);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("file-size check step present"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_only_max_file_size_threshold_is_referenced() {
    let results = run_case(
        r#"
MAX_FILE_SIZE=1048576
if [ "$file_size" -gt "$MAX_FILE_SIZE" ]; then
    exit 1
fi
"#,
    );
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("file-size check step missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_git_cat_file_size_is_only_echoed() {
    let results = run_case(r#"echo "git cat-file -s :$file""#);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("file-size check step missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
