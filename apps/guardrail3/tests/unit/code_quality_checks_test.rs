use std::path::Path;

use guardrail3::app::rs::validate::code_quality_checks::*;
use guardrail3::domain::report::Severity;

// ---- Bug 5: R58 direct std::fs detection ----

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r58_catches_use_std_fs() {
    let content = "use std::fs;\nfn main() {}";
    let path = Path::new("src/foo.rs");
    let mut results = Vec::new();
    check_direct_fs_usage(path, content, false, &mut results);
    assert!(!results.is_empty(), "Should catch 'use std::fs'");
    assert_eq!(results[0].id, "R58");
}

#[test]
fn r58_allows_fs_module() {
    let content = "use std::fs;\nfn main() {}";
    let path = Path::new("src/fs.rs");
    let mut results = Vec::new();
    check_direct_fs_usage(path, content, false, &mut results);
    assert!(results.is_empty(), "fs.rs should be exempt from R58");
}

#[test]
fn r58_allows_string_literals_in_modules() {
    let content = r#"let ban = "std::fs::read_to_string";"#;
    let path = Path::new("src/modules/clippy.rs");
    let mut results = Vec::new();
    check_direct_fs_usage(path, content, false, &mut results);
    assert!(
        results.is_empty(),
        "String literals in modules/ should be exempt"
    );
}

#[test]
fn r58_catches_type_method_call() {
    // std::fs::Permissions::from_mode IS a std::fs call in expression context — should be caught
    let content = "fn foo() { let p = std::fs::Permissions::from_mode(0o755); }";
    let path = Path::new("src/foo.rs");
    let mut results = Vec::new();
    check_direct_fs_usage(path, content, false, &mut results);
    assert!(
        !results.is_empty(),
        "std::fs::Permissions::from_mode in expression context should be caught"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r58_catches_read_to_string() {
    let content = "fn foo() { let s = std::fs::read_to_string(\"x\").unwrap(); }";
    let path = Path::new("src/foo.rs");
    let mut results = Vec::new();
    check_direct_fs_usage(path, content, false, &mut results);
    assert!(
        !results.is_empty(),
        "Direct std::fs::read_to_string should be caught"
    );
    assert_eq!(results[0].id, "R58");
}

#[test]
fn r58_allows_metadata_type() {
    // Type in function signature — syn treats this as Type::Path, not Expr::Path
    let content = "fn check(fs: &dyn FileSystem, m: std::fs::Metadata) {}";
    let path = Path::new("src/foo.rs");
    let mut results = Vec::new();
    check_direct_fs_usage(path, content, false, &mut results);
    assert!(
        results.is_empty(),
        "std::fs::Metadata type reference should be exempt"
    );
}

#[test]
fn r58_skips_cfg_test_block() {
    let content = "\
fn production_code() {}

#[cfg(test)]
mod tests {
    use std::fs;
    fn helper() {
        let _ = std::fs::read_to_string(\"test.txt\");
    }
}";
    let path = Path::new("src/foo.rs");
    let mut results = Vec::new();
    check_direct_fs_usage(path, content, false, &mut results);
    assert!(
        results.is_empty(),
        "std::fs usage inside #[cfg(test)] block should not trigger R58, got: {results:?}"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r58_still_catches_production_fs_before_cfg_test() {
    let content = "\
use std::fs;

fn production_code() {}

#[cfg(test)]
mod tests {
    use std::fs;
}";
    let path = Path::new("src/foo.rs");
    let mut results = Vec::new();
    check_direct_fs_usage(path, content, false, &mut results);
    assert_eq!(
        results.len(),
        1,
        "Should catch production std::fs but not the one in #[cfg(test)]"
    );
    assert_eq!(results[0].id, "R58");
    assert_eq!(
        results[0].line,
        Some(1),
        "Should flag line 1 (production code), not line 7 (test code)"
    );
}

// ---- R43 todo macro tests ----

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn todo_macro_produces_warn() {
    let content = "fn foo() { todo!(); }";
    let path = Path::new("src/foo.rs");
    let mut results = Vec::new();
    check_todo_macros(path, content, false, &mut results);
    assert!(!results.is_empty());
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].id, "R43");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn path_qualified_todo_macro_produces_warn() {
    let content = "fn foo() { std::todo!(); }";
    let path = Path::new("src/foo.rs");
    let mut results = Vec::new();
    check_todo_macros(path, content, false, &mut results);
    assert!(!results.is_empty(), "std::todo!() should be caught by R43");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].id, "R43");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn core_unimplemented_macro_produces_warn() {
    let content = "fn foo() { core::unimplemented!(); }";
    let path = Path::new("src/foo.rs");
    let mut results = Vec::new();
    check_todo_macros(path, content, false, &mut results);
    assert!(
        !results.is_empty(),
        "core::unimplemented!() should be caught by R43"
    );
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].id, "R43");
}
