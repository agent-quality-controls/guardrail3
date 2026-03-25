use std::path::Path;

use guardrail3_app_ts::validate::ts_comment_checks::{check_eslint_disable, check_ts_ignore};
use guardrail3_domain_report::Severity;

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn test_eslint_disable_block_no_reason_t23() {
    let content = "const x = 1;\n/* eslint-disable no-console */\nconsole.log(x);\n";
    let path = Path::new("test.ts");
    let mut results = Vec::new();
    check_eslint_disable(path, content, &mut results);
    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.id == "T23" && r.severity == Severity::Error)
        .collect();
    assert!(!errors.is_empty(), "Should produce T23 Error");
    assert_eq!(errors[0].id, "T23");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn test_eslint_disable_block_with_reason_t24() {
    let content =
        "const x = 1;\n/* eslint-disable no-console -- needed for CLI */\nconsole.log(x);\n";
    let path = Path::new("test.ts");
    let mut results = Vec::new();
    check_eslint_disable(path, content, &mut results);
    let infos: Vec<_> = results
        .iter()
        .filter(|r| r.id == "T24" && r.severity == Severity::Info)
        .collect();
    assert!(!infos.is_empty(), "Should produce T24 Info");
    assert_eq!(infos[0].id, "T24");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn test_eslint_disable_next_line_no_reason_t25() {
    let content = "// eslint-disable-next-line no-console\nconsole.log('hello');\n";
    let path = Path::new("test.ts");
    let mut results = Vec::new();
    check_eslint_disable(path, content, &mut results);
    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.id == "T25" && r.severity == Severity::Error)
        .collect();
    assert!(!errors.is_empty(), "Should produce T25 Error");
    assert_eq!(errors[0].id, "T25");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn test_eslint_disable_next_line_with_reason_t26() {
    let content = "// eslint-disable-next-line no-console -- reason\nconsole.log('hello');\n";
    let path = Path::new("test.ts");
    let mut results = Vec::new();
    check_eslint_disable(path, content, &mut results);
    let infos: Vec<_> = results
        .iter()
        .filter(|r| r.id == "T26" && r.severity == Severity::Info)
        .collect();
    assert!(!infos.is_empty(), "Should produce T26 Info");
    assert_eq!(infos[0].id, "T26");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn test_ts_ignore_t27() {
    let content = "// @ts-ignore\nconst x: number = 'hello';\n";
    let path = Path::new("test.ts");
    let mut results = Vec::new();
    check_ts_ignore(path, content, &mut results);
    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.id == "T27" && r.severity == Severity::Error)
        .collect();
    assert!(!errors.is_empty(), "Should produce T27 Error");
    assert_eq!(errors[0].id, "T27");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn test_ts_expect_error_no_explanation_t28() {
    let content = "// @ts-expect-error\nconst x: number = 'hello';\n";
    let path = Path::new("test.ts");
    let mut results = Vec::new();
    check_ts_ignore(path, content, &mut results);
    let warns: Vec<_> = results
        .iter()
        .filter(|r| r.id == "T28" && r.severity == Severity::Warn)
        .collect();
    assert!(!warns.is_empty(), "Should produce T28 Warn");
    assert_eq!(warns[0].id, "T28");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn test_ts_expect_error_with_explanation_t29() {
    let content = "// @ts-expect-error: type mismatch\nconst x: number = 'hello';\n";
    let path = Path::new("test.ts");
    let mut results = Vec::new();
    check_ts_ignore(path, content, &mut results);
    let infos: Vec<_> = results
        .iter()
        .filter(|r| r.id == "T29" && r.severity == Severity::Info)
        .collect();
    assert!(!infos.is_empty(), "Should produce T29 Info");
    assert_eq!(infos[0].id, "T29");
}
