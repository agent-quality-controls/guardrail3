use guardrail3_app_rs_ast::ast_helpers::find_derive_attributes;
use guardrail3_app_rs_legacy_validate::garde_checks::{
    EXPECTED_AXUM_TYPE_BANS, EXPECTED_SERDE_METHOD_BANS, check_ban_presence,
    check_garde_dependency, check_reqwest_json_ban_from_table, count_unvalidated_input_structs,
};
use guardrail3_domain_report::{CheckResult, Severity};

// Test-only helpers that parse clippy.toml content strings

fn check_serde_method_bans(content: &str, file: &str) -> Vec<CheckResult> {
    match content.parse::<toml::Value>() {
        Ok(table) => check_ban_presence(
            &table,
            "disallowed-methods",
            EXPECTED_SERDE_METHOD_BANS,
            "R-GARDE-02",
            "Garde serde method ban",
            file,
        ),
        Err(_) => vec![],
    }
}

fn check_axum_type_bans(content: &str, file: &str) -> Vec<CheckResult> {
    match content.parse::<toml::Value>() {
        Ok(table) => check_ban_presence(
            &table,
            "disallowed-types",
            EXPECTED_AXUM_TYPE_BANS,
            "R-GARDE-03",
            "Garde axum type ban",
            file,
        ),
        Err(_) => vec![],
    }
}

fn check_reqwest_json_ban(content: &str, file: &str) -> Vec<CheckResult> {
    match content.parse::<toml::Value>() {
        Ok(table) => check_reqwest_json_ban_from_table(&table, file),
        Err(_) => vec![],
    }
}

// ---------------------------------------------------------------------------
// R-GARDE-01 tests
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r_garde_01_missing_garde_dependency() {
    let cargo = r#"
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1" }
"#;
    let results = check_garde_dependency(Some(cargo));
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "R-GARDE-01");
    assert!(
        results[0].message.contains("not in"),
        "Should indicate garde is missing"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r_garde_01_has_garde_dependency() {
    let cargo = r#"
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
garde = { version = "0.20", features = ["derive"] }
"#;
    let results = check_garde_dependency(Some(cargo));
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "R-GARDE-01");
    assert!(
        results[0].title.contains("found"),
        "Should indicate garde is present"
    );
}

// ---------------------------------------------------------------------------
// R-GARDE-02 tests
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r_garde_02_no_serde_bans() {
    let clippy = r#"
disallowed-methods = [
    { path = "std::env::var", reason = "banned" },
]
"#;
    let results = check_serde_method_bans(clippy, "clippy.toml");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "R-GARDE-02");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(
        results[0].message.contains("serde_json::from_str"),
        "Should list missing methods"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r_garde_02_all_serde_bans_present() {
    let clippy = r#"
disallowed-methods = [
    { path = "serde_json::from_str", reason = "Use Validated<T>::new()" },
    { path = "serde_json::from_slice", reason = "Use Validated<T>::new()" },
    { path = "serde_json::from_value", reason = "Use Validated<T>::new()" },
    { path = "serde_json::from_reader", reason = "Use Validated<T>::new()" },
    { path = "toml::from_str", reason = "Use Validated<T>::new()" },
    { path = "serde_yaml::from_str", reason = "Use Validated<T>::new()" },
    { path = "serde_yaml::from_reader", reason = "Use Validated<T>::new()" },
]
"#;
    let results = check_serde_method_bans(clippy, "clippy.toml");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "R-GARDE-02");
    assert_eq!(results[0].severity, Severity::Info);
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r_garde_02_partial_serde_bans() {
    let clippy = r#"
disallowed-methods = [
    { path = "serde_json::from_str", reason = "banned" },
    { path = "serde_json::from_slice", reason = "banned" },
]
"#;
    let results = check_serde_method_bans(clippy, "clippy.toml");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].message.contains("serde_json::from_value"));
    assert!(results[0].message.contains("serde_json::from_reader"));
    assert!(results[0].message.contains("toml::from_str"));
    // Already present ones should NOT appear in missing list
    assert!(!results[0].message.contains("serde_json::from_str"));
}

// ---------------------------------------------------------------------------
// R-GARDE-03 tests
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r_garde_03_no_axum_bans() {
    let clippy = r#"
disallowed-types = [
    { path = "std::collections::HashMap", reason = "banned" },
]
"#;
    let results = check_axum_type_bans(clippy, "clippy.toml");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "R-GARDE-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].message.contains("axum::extract::Json"));
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r_garde_03_all_axum_bans_present() {
    let clippy = r#"
disallowed-types = [
    { path = "axum::extract::Json", reason = "Use ValidatedJson" },
    { path = "axum::Json", reason = "Use ValidatedJson" },
    { path = "axum::extract::Query", reason = "Use ValidatedQuery" },
    { path = "axum::extract::Form", reason = "Use ValidatedForm" },
]
"#;
    let results = check_axum_type_bans(clippy, "clippy.toml");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "R-GARDE-03");
    assert_eq!(results[0].severity, Severity::Info);
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r_garde_03_partial_axum_bans() {
    let clippy = r#"
disallowed-types = [
    { path = "axum::extract::Json", reason = "Use ValidatedJson" },
    { path = "axum::Json", reason = "Use ValidatedJson" },
]
"#;
    let results = check_axum_type_bans(clippy, "clippy.toml");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].message.contains("axum::extract::Query"));
    assert!(results[0].message.contains("axum::extract::Form"));
    assert!(!results[0].message.contains("axum::extract::Json"));
}

// ---------------------------------------------------------------------------
// R-GARDE-04 tests
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r_garde_04_missing_reqwest_ban() {
    let clippy = r#"
disallowed-methods = [
    { path = "std::env::var", reason = "banned" },
]
"#;
    let results = check_reqwest_json_ban(clippy, "clippy.toml");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "R-GARDE-04");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].message.contains("reqwest::Response::json"));
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn r_garde_04_present_reqwest_ban() {
    let clippy = r#"
disallowed-methods = [
    { path = "reqwest::Response::json", reason = "Use Validated<T>::new()" },
]
"#;
    let results = check_reqwest_json_ban(clippy, "clippy.toml");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "R-GARDE-04");
    assert_eq!(results[0].severity, Severity::Info);
}

// ---------------------------------------------------------------------------
// R-GARDE-05 tests
// ---------------------------------------------------------------------------

#[test]
fn r_garde_05_counts_deserialize_structs_ast() {
    let content_both = r"
use serde::Deserialize;

#[derive(Deserialize, garde::Validate)]
struct Foo {
    name: String,
}

#[derive(Deserialize)]
struct Bar {
    label: String,
}

#[derive(Serialize)]
struct Baz {
    id: i64,
}
";
    #[allow(clippy::expect_used)] // reason: test — panic on parse failure is correct
    let parsed = syn::parse_file(content_both).expect("should parse");
    let derives = find_derive_attributes(&parsed);
    let (with, without) = count_unvalidated_input_structs(&derives);
    assert_eq!(with, 1, "Foo has both Deserialize + Validate");
    assert_eq!(
        without, 1,
        "Bar has Deserialize without Validate (has non-primitive field)"
    );
}

#[test]
fn parser_without_validate_flagged() {
    let content = r"
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short)]
    name: String,
}
";
    #[allow(clippy::expect_used)] // reason: test — panic on parse failure is correct
    let parsed = syn::parse_file(content).expect("should parse");
    let derives = find_derive_attributes(&parsed);
    let (with, without) = count_unvalidated_input_structs(&derives);
    assert_eq!(with, 0);
    assert_eq!(without, 1, "Parser without Validate should be flagged");
}

#[test]
fn args_without_validate_flagged() {
    let content = r"
use clap::Args;

#[derive(Args)]
struct SubCmd {
    #[arg(long)]
    output: String,
}
";
    #[allow(clippy::expect_used)] // reason: test — panic on parse failure is correct
    let parsed = syn::parse_file(content).expect("should parse");
    let derives = find_derive_attributes(&parsed);
    let (with, without) = count_unvalidated_input_structs(&derives);
    assert_eq!(with, 0);
    assert_eq!(without, 1, "Args without Validate should be flagged");
}

#[test]
fn from_row_without_validate_flagged() {
    let content = r"
use sqlx::FromRow;

#[derive(FromRow)]
struct UserRow {
    id: i64,
    name: String,
}
";
    #[allow(clippy::expect_used)] // reason: test — panic on parse failure is correct
    let parsed = syn::parse_file(content).expect("should parse");
    let derives = find_derive_attributes(&parsed);
    let (with, without) = count_unvalidated_input_structs(&derives);
    assert_eq!(with, 0);
    assert_eq!(without, 1, "FromRow without Validate should be flagged");
}

#[test]
fn parser_with_validate_ok() {
    let content = r"
use clap::Parser;

#[derive(Parser, garde::Validate)]
struct Cli {
    #[arg(short)]
    name: String,
}
";
    #[allow(clippy::expect_used)] // reason: test — panic on parse failure is correct
    let parsed = syn::parse_file(content).expect("should parse");
    let derives = find_derive_attributes(&parsed);
    let (with, without) = count_unvalidated_input_structs(&derives);
    assert_eq!(with, 1, "Parser + Validate should be counted as validated");
    assert_eq!(without, 0);
}

#[test]
fn deserialize_with_validate_ok() {
    let content = r"
use serde::Deserialize;

#[derive(Deserialize, garde::Validate)]
struct Input {
    name: String,
}
";
    #[allow(clippy::expect_used)] // reason: test — panic on parse failure is correct
    let parsed = syn::parse_file(content).expect("should parse");
    let derives = find_derive_attributes(&parsed);
    let (with, without) = count_unvalidated_input_structs(&derives);
    assert_eq!(
        with, 1,
        "Deserialize + Validate should be counted as validated"
    );
    assert_eq!(without, 0);
}

#[test]
fn garde_05_all_bool_struct_no_validate_needed() {
    let content = r"
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    enabled: bool,
    active: bool,
    count: u32,
}
";
    #[allow(clippy::expect_used)] // reason: test — panic on parse failure is correct
    let parsed = syn::parse_file(content).expect("should parse");
    let derives = find_derive_attributes(&parsed);
    let (with, without) = count_unvalidated_input_structs(&derives);
    assert_eq!(
        with, 0,
        "All-primitive struct should not be counted as needing Validate"
    );
    assert_eq!(
        without, 0,
        "All-primitive struct should not be flagged as missing Validate"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn garde_missing_is_error() {
    let cargo = r#"
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
"#;
    let results = check_garde_dependency(Some(cargo));
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "R-GARDE-01");
    assert_eq!(
        results[0].severity,
        Severity::Error,
        "Missing garde dependency must be Error severity"
    );
}
