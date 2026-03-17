//! Tests extracted from `app::ts::validate::test_checks`
#![allow(
    clippy::expect_used,
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::manual_assert
)] // reason: test crate

use std::fs as stdfs;

use guardrail3::app::ts::validate::test_checks::{
    check_only_in_source_content, check_skip_without_reason_content, check_stryker_config,
    check_test_files_exist, check_test_runner_config,
};
use guardrail3::domain::report::Severity;

fn make_temp_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("failed to create temp dir")
}

// ---- T-TEST-01: Stryker config ----

#[test]
fn t_test_01_no_stryker_config() {
    let dir = make_temp_dir();
    let mut results = Vec::new();
    check_stryker_config(dir.path(), &mut results);
    assert_eq!(results.len(), 1, "Should produce exactly one result");
    assert_eq!(results[0].id, "T-TEST-01");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn t_test_01_stryker_config_exists() {
    let dir = make_temp_dir();
    stdfs::write(dir.path().join("stryker.config.json"), "{}").expect("write config");
    let mut results = Vec::new();
    check_stryker_config(dir.path(), &mut results);
    assert_eq!(results.len(), 1, "Should produce exactly one result");
    assert_eq!(results[0].id, "T-TEST-01");
    assert_eq!(results[0].severity, Severity::Info);
}

// ---- T-TEST-02: Test files exist ----

#[test]
fn t_test_02_no_test_files() {
    let dir = make_temp_dir();
    let mut results = Vec::new();
    check_test_files_exist(dir.path(), &mut results);
    assert_eq!(results.len(), 1, "Should produce exactly one result");
    assert_eq!(results[0].id, "T-TEST-02");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn t_test_02_test_file_exists() {
    let dir = make_temp_dir();
    stdfs::write(dir.path().join("foo.test.ts"), "test('a', () => {})").expect("write test file");
    let mut results = Vec::new();
    check_test_files_exist(dir.path(), &mut results);
    assert_eq!(results.len(), 1, "Should produce exactly one result");
    assert_eq!(results[0].id, "T-TEST-02");
    assert_eq!(results[0].severity, Severity::Info);
}

// ---- T-TEST-03: Test runner config ----

#[test]
fn t_test_03_no_runner_config() {
    let dir = make_temp_dir();
    let mut results = Vec::new();
    check_test_runner_config(dir.path(), &mut results);
    assert_eq!(results.len(), 1, "Should produce exactly one result");
    assert_eq!(results[0].id, "T-TEST-03");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn t_test_03_vitest_config_exists() {
    let dir = make_temp_dir();
    stdfs::write(dir.path().join("vitest.config.ts"), "export default {}").expect("write config");
    let mut results = Vec::new();
    check_test_runner_config(dir.path(), &mut results);
    assert_eq!(results.len(), 1, "Should produce exactly one result");
    assert_eq!(results[0].id, "T-TEST-03");
    assert_eq!(results[0].severity, Severity::Info);
}

// ---- T-TEST-04: .skip() without reason ----

#[test]
fn t_test_04_bare_skip_flagged() {
    let content = "test.skip('broken test', () => {\n  expect(1).toBe(1);\n});";
    let results = check_skip_without_reason_content(content, "app.test.ts");
    assert_eq!(results.len(), 1, "Should flag one skip");
    assert_eq!(results[0].id, "T-TEST-04");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn t_test_04_skip_with_reason_ok() {
    let content = "test.skip('broken test', () => {}); // reason: flaky on CI, tracked in #123";
    let results = check_skip_without_reason_content(content, "app.test.ts");
    assert_eq!(results.len(), 1, "Should produce info for skip with reason");
    assert_eq!(results[0].id, "T-TEST-04");
    assert_eq!(results[0].severity, Severity::Info);
}

// ---- T-TEST-05: .only() in committed code ----

#[test]
fn t_test_05_only_flagged() {
    let content = "describe.only('my suite', () => {\n  it('works', () => {});\n});";
    let results = check_only_in_source_content(content, "app.test.ts");
    assert_eq!(results.len(), 1, "Should flag one .only()");
    assert_eq!(results[0].id, "T-TEST-05");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn t_test_05_clean_source() {
    let content = "describe('my suite', () => {\n  it('works', () => {});\n});";
    let results = check_only_in_source_content(content, "app.test.ts");
    assert!(
        results.is_empty(),
        "Clean source should produce no T-TEST-05 results"
    );
}

// ---- T-TEST-04 tree-sitter: string false-positive rejection ----

#[test]
fn t_test_04_skip_in_string_not_flagged() {
    let content = "const s = \"test.skip('broken', () => {})\";\nexport default s;";
    let results = check_skip_without_reason_content(content, "app.test.ts");
    assert!(
        results.is_empty(),
        "test.skip inside string should not be flagged (tree-sitter)"
    );
}

#[test]
fn t_test_04_skip_in_comment_not_flagged() {
    let content = "// test.skip('broken', () => {})\nconst x = 1;";
    let results = check_skip_without_reason_content(content, "app.test.ts");
    assert!(
        results.is_empty(),
        "test.skip inside comment should not be flagged (tree-sitter)"
    );
}

#[test]
fn t_test_04_skip_in_template_not_flagged() {
    let content = "const s = `test.skip('broken', () => {})`;\nexport default s;";
    let results = check_skip_without_reason_content(content, "app.test.ts");
    assert!(
        results.is_empty(),
        "test.skip inside template literal should not be flagged (tree-sitter)"
    );
}

// ---- T-TEST-04 / T-TEST-05: TSX files parse with TSX grammar, not TS ----

#[test]
fn t_test_04_tsx_jsx_content_uses_treesitter() {
    let content = "const App = () => <div>test</div>;\ndescribe.skip('test', () => {});";
    let results = check_skip_without_reason_content(content, "app.test.tsx");
    assert_eq!(
        results.len(),
        1,
        "Should detect .skip() in TSX via tree-sitter"
    );
    assert_eq!(results[0].id, "T-TEST-04");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].line, Some(2));
}

#[test]
fn t_test_04_tsx_skip_in_string_not_flagged() {
    let content =
        "const App = () => <div>{\"test.skip('broken', () => {})\"}</div>;\nexport default App;";
    let results = check_skip_without_reason_content(content, "app.test.tsx");
    assert!(
        results.is_empty(),
        "test.skip inside JSX string should not be flagged when TSX grammar is used"
    );
}

#[test]
fn t_test_05_tsx_jsx_content_uses_treesitter() {
    let content = "const App = () => <div>test</div>;\nit.only('test', () => {});";
    let results = check_only_in_source_content(content, "app.test.tsx");
    assert_eq!(
        results.len(),
        1,
        "Should detect .only() in TSX via tree-sitter"
    );
    assert_eq!(results[0].id, "T-TEST-05");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].line, Some(2));
}

#[test]
fn t_test_05_tsx_only_in_string_not_flagged() {
    let content =
        "const App = () => <div>{\"describe.only('suite', () => {})\"}</div>;\nexport default App;";
    let results = check_only_in_source_content(content, "app.test.tsx");
    assert!(
        results.is_empty(),
        "describe.only inside JSX string should not be flagged when TSX grammar is used"
    );
}

// ---- T-TEST-05 tree-sitter: string false-positive rejection ----

#[test]
fn t_test_05_only_in_string_not_flagged() {
    let content = "const s = \"describe.only('suite', () => {})\";\nexport default s;";
    let results = check_only_in_source_content(content, "app.test.ts");
    assert!(
        results.is_empty(),
        "describe.only inside string should not be flagged (tree-sitter)"
    );
}

#[test]
fn t_test_05_only_in_comment_not_flagged() {
    let content = "// it.only('test', () => {})\nconst x = 1;";
    let results = check_only_in_source_content(content, "app.test.ts");
    assert!(
        results.is_empty(),
        "it.only inside comment should not be flagged (tree-sitter)"
    );
}

#[test]
fn t_test_05_only_multiple_detected() {
    let content = "it.only('a', () => {});\ndescribe.only('b', () => {});";
    let results = check_only_in_source_content(content, "app.test.ts");
    assert_eq!(results.len(), 2, "Should flag both .only() calls");
    assert_eq!(results[0].id, "T-TEST-05");
    assert_eq!(results[1].id, "T-TEST-05");
}
