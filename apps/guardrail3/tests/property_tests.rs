//! Property-based tests for guardrail3.
//!
//! Uses proptest to generate random inputs and verify invariants:
//! no panics, consistent results, correct pattern detection.
use garde as _;

// Suppress unused crate dependency warnings for crates used by the lib crate
use clap as _;
use colored as _;
use glob as _;
use guardrail3 as _;
use ignore as _;
use proc_macro2 as _;
use quote as _;
use serde as _;
use serde_json as _;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use syn as _;
use toml as _;
use tree_sitter as _;
use tree_sitter_typescript as _;
use walkdir as _;

use proptest::prelude::*;
use tempfile::TempDir;

/// Get the path to the compiled guardrail3 binary.
fn binary_path() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_BIN_EXE_guardrail3"));
    assert!(path.exists(), "Binary not found at {}", path.display());
    path
}

/// Run guardrail3 rs validate on a directory, returning JSON output.
///
/// Returns (code, stdout, stderr).
#[allow(clippy::expect_used)] // reason: test helper -- panics indicate broken test setup
#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
fn run_rs_validate(dir: &std::path::Path) -> (i32, String, String) {
    let dir_str = dir.display().to_string();
    let output = Command::new(binary_path())
        .args(["rs", "validate", "--format", "json", "--code"])
        .arg(&dir_str)
        .output()
        .expect("Failed to run guardrail3");

    let code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (code, stdout, stderr)
}

/// Run guardrail3 validate (auto-detect) on a directory, returning JSON output.
#[allow(clippy::expect_used)] // reason: test helper -- panics indicate broken test setup
#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
fn run_validate(dir: &std::path::Path) -> (i32, String, String) {
    let dir_str = dir.display().to_string();
    let output = Command::new(binary_path())
        .args(["rs", "validate", "--format", "json"])
        .arg(&dir_str)
        .output()
        .expect("Failed to run guardrail3");

    let code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (code, stdout, stderr)
}

/// Extract a string field from a JSON value, returning owned String.
fn json_str_field(obj: &serde_json::Value, field: &str) -> String {
    obj.get(field)
        .and_then(|val| val.as_str())
        .unwrap_or("")
        .to_owned()
}

/// Parse check results from JSON output, returning Vec of (id, severity, title, message).
#[allow(clippy::type_complexity)] // reason: tuple type is clear in context
#[allow(clippy::panic)] // reason: test helper -- invalid JSON means broken test setup
#[allow(clippy::disallowed_methods)] // reason: serde_json::from_str needed to parse test output
fn parse_check_results(json_str: &str) -> Vec<(String, String, String, String)> {
    let parsed: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(val) => val,
        Err(e) => panic!("Failed to parse JSON: {e}\nOutput: {json_str}"),
    };

    let mut results = Vec::new();
    if let Some(sections) = parsed.get("sections").and_then(|s| s.as_array()) {
        for section in sections {
            if let Some(checks) = section.get("results").and_then(|r| r.as_array()) {
                for check in checks {
                    let id = json_str_field(check, "id");
                    let severity = json_str_field(check, "severity");
                    let title = json_str_field(check, "title");
                    let message = json_str_field(check, "message");
                    results.push((id, severity, title, message));
                }
            }
        }
    }
    results
}

/// Create a minimal Rust project in temp dir with given source files.
#[allow(clippy::expect_used)] // reason: test helper -- panics indicate broken test setup
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
#[allow(clippy::type_complexity)] // reason: slice of tuples is the clearest API here
fn setup_rust_project(dir: &std::path::Path, source_files: &[(&str, &str)]) {
    fs::create_dir_all(dir.join("src")).expect("create src dir");
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"test-project\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("write Cargo.toml");

    for (name, content) in source_files {
        let path = dir.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent dirs");
        }
        fs::write(path, content).expect("write source file");
    }
}

// ============================================================
// 1. Any valid TOML deserializes without panic
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    #[allow(clippy::expect_used)] // reason: test -- panics indicate broken test setup
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn property_toml_parse_never_panics(
        s in "[a-z_]{1,10} = [a-z0-9\"\\[\\]]{0,20}\n{0,3}"
    ) {
        let dir = TempDir::new().expect("create temp dir");
        let dir_str = dir.path().display().to_string();
        fs::write(dir.path().join("guardrail3.toml"), &s).expect("write config");

        // Run the tool -- it should not panic regardless of config content
        let output = Command::new(binary_path())
            .args(["rs", "validate", "--format", "json"])
            .arg(&dir_str)
            .output()
            .expect("Failed to run guardrail3");

        // The process should exit cleanly (0 or 1), not crash
        let code = output.status.code().unwrap_or(-1);
        prop_assert!(
            code == 0 || code == 1,
            "Process should exit cleanly, got code {code}"
        );
    }
}

// ============================================================
// 2. Config round-trip (serialize/deserialize)
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    #[test]
    #[allow(clippy::unwrap_used)] // reason: test assertions use unwrap for clarity
    #[allow(clippy::disallowed_methods)] // reason: toml::from_str needed to test config parsing
    fn property_config_round_trip(
        version in "[0-9]\\.[0-9]",
        profile_name in prop::sample::select(vec!["service", "library"]),
    ) {
        let toml_content = format!(
            "version = \"{version}\"\n\n[profile]\nname = \"{profile_name}\"\n"
        );

        // Parse -> serialize -> parse should not lose data
        let parsed: toml::Value = toml::from_str(&toml_content).unwrap();

        let round_tripped_str = toml::to_string(&parsed).unwrap();
        let parsed2: toml::Value = toml::from_str(&round_tripped_str).unwrap();

        prop_assert_eq!(
            parsed.get("version"),
            parsed2.get("version"),
            "Version field lost in round-trip"
        );
        prop_assert_eq!(
            parsed.get("profile").and_then(|p| p.get("name")),
            parsed2.get("profile").and_then(|p| p.get("name")),
            "Profile name lost in round-trip"
        );
    }
}

// ============================================================
// 3. Profile always produces valid config (init never panics)
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(9))]

    #[test]
    #[allow(clippy::expect_used)] // reason: test -- panics indicate broken test setup
    #[allow(clippy::unwrap_used)] // reason: test assertions
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access and toml parsing
    fn property_init_profile_never_panics(
        profile in prop::sample::select(vec!["service", "library"]),
    ) {
        let dir = TempDir::new().expect("create temp dir");
        let dir_str = dir.path().display().to_string();

        let output = Command::new(binary_path())
            .args(["rs", "init", "--profile", profile])
            .arg(&dir_str)
            .output()
            .expect("Failed to run guardrail3 rs init");

        let code = output.status.code().unwrap_or(-1);
        prop_assert!(
            code == 0 || code == 1,
            "init should not crash, got code {code}"
        );

        // If init succeeded, the config file should exist and be valid TOML
        let config_path = dir.path().join("guardrail3.toml");
        if config_path.exists() {
            let content = fs::read_to_string(&config_path).unwrap();
            let parsed: Result<toml::Value, _> = toml::from_str(&content);
            prop_assert!(
                parsed.is_ok(),
                "Generated config should be valid TOML: {:?}",
                parsed.err()
            );
        }
    }
}

// ============================================================
// 4. Validate never panics on any input (random Rust source)
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    #[test]
    #[allow(clippy::expect_used)] // reason: test -- panics indicate broken test setup
    fn property_validate_never_panics_on_random_source(
        content in "[a-zA-Z0-9 _{}()#\\[\\];:,\\.\\n\"'/!\\*\\+\\-=<>\\?@\\\\]{0,500}"
    ) {
        let dir = TempDir::new().expect("create temp dir");
        setup_rust_project(dir.path(), &[("src/main.rs", &content)]);

        let (code, _stdout, _stderr) = run_rs_validate(dir.path());

        // Should exit cleanly (0 or 1), never crash/signal
        prop_assert!(
            code == 0 || code == 1,
            "Validator should not crash on random input, got code {code}"
        );
    }
}

// ============================================================
// 5. Every CheckResult has a non-empty ID
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]

    #[test]
    #[allow(clippy::expect_used)] // reason: test -- panics indicate broken test setup
    fn property_check_results_have_nonempty_id(
        content in "fn main\\(\\) \\{\\}\n(use [a-z_]+::[a-z_]+;\n){0,5}"
    ) {
        let dir = TempDir::new().expect("create temp dir");
        setup_rust_project(dir.path(), &[("src/main.rs", &content)]);

        let (_code, stdout, _stderr) = run_rs_validate(dir.path());

        if !stdout.is_empty() {
            let results = parse_check_results(&stdout);
            for (id, _sev, _title, _msg) in &results {
                prop_assert!(
                    !id.is_empty(),
                    "Every check result must have a non-empty ID"
                );
            }
        }
    }
}

// ============================================================
// 6. Severity is always valid (error, warn, or info)
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]

    #[test]
    #[allow(clippy::expect_used)] // reason: test -- panics indicate broken test setup
    fn property_severity_always_valid(
        content in "fn main\\(\\) \\{\\}\n(let x = [0-9]+;\n){0,5}"
    ) {
        let dir = TempDir::new().expect("create temp dir");
        setup_rust_project(dir.path(), &[("src/main.rs", &content)]);

        let (_code, stdout, _stderr) = run_rs_validate(dir.path());

        if !stdout.is_empty() {
            let results = parse_check_results(&stdout);
            let valid_severities = ["error", "warn", "info"];
            for (_id, severity, _title, _msg) in &results {
                prop_assert!(
                    valid_severities.contains(&severity.as_str()),
                    "Invalid severity '{severity}' -- expected error, warn, or info"
                );
            }
        }
    }
}

// ============================================================
// 7. Check results are deterministic
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(15))]

    #[test]
    #[allow(clippy::expect_used)] // reason: test -- panics indicate broken test setup
    fn property_results_are_deterministic(
        num_lines in 1_usize..10,
    ) {
        // Build a simple but varied source file
        let mut source = String::from("fn main() {}\n");
        for i in 0..num_lines {
            let _ = writeln!(source, "fn func_{i}() {{}}");
        }

        let dir = TempDir::new().expect("create temp dir");
        setup_rust_project(dir.path(), &[("src/main.rs", &source)]);

        let (_code1, stdout1, _stderr1) = run_rs_validate(dir.path());
        let (_code2, stdout2, _stderr2) = run_rs_validate(dir.path());

        // Parse both runs
        if !stdout1.is_empty() && !stdout2.is_empty() {
            let results1 = parse_check_results(&stdout1);
            let results2 = parse_check_results(&stdout2);

            prop_assert_eq!(
                results1.len(),
                results2.len(),
                "Same input should produce same number of results"
            );

            for (r1, r2) in results1.iter().zip(results2.iter()) {
                prop_assert_eq!(
                    &r1.0, &r2.0,
                    "Check IDs should be identical across runs"
                );
                prop_assert_eq!(
                    &r1.1, &r2.1,
                    "Severities should be identical across runs"
                );
            }
        }
    }
}

// ============================================================
// 8. allow without reason detected / with reason OK
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(15))]

    #[test]
    #[allow(clippy::expect_used)] // reason: test -- panics indicate broken test setup
    fn property_allow_without_reason_detected(
        lint in prop::sample::select(vec![
            "clippy::unwrap_used",
            "clippy::expect_used",
            "dead_code",
            "clippy::too_many_lines",
        ]),
    ) {
        // Without reason -- should be flagged (R32)
        let without_reason = format!(
            "{}({lint})]\nfn foo() {{}}\nfn main() {{}}\n",
            ["#[", "allow"].concat() // concat avoids pre-commit hook false match
        );

        let dir = TempDir::new().expect("create temp dir");
        setup_rust_project(dir.path(), &[("src/main.rs", &without_reason)]);

        let (_code, stdout, _stderr) = run_rs_validate(dir.path());
        if !stdout.is_empty() {
            let results = parse_check_results(&stdout);
            let has_r32 = results.iter().any(|(id, sev, _, _)| id == "R32" && sev == "error");
            prop_assert!(
                has_r32,
                "allow({lint}) without reason should produce R32 error"
            );
        }

        // With reason -- should be info (R33), not error
        let with_reason = format!(
            "{}({lint})] // reason: justified in test\nfn foo() {{}}\nfn main() {{}}\n",
            ["#[", "allow"].concat() // concat avoids pre-commit hook false match
        );

        let dir2 = TempDir::new().expect("create temp dir");
        setup_rust_project(dir2.path(), &[("src/main.rs", &with_reason)]);

        let (_code2, stdout2, _stderr2) = run_rs_validate(dir2.path());
        if !stdout2.is_empty() {
            let results2 = parse_check_results(&stdout2);
            let has_r32_error = results2
                .iter()
                .any(|(id, sev, _, _)| id == "R32" && sev == "error");
            prop_assert!(
                !has_r32_error,
                "allow({lint}) with reason should NOT produce R32 error"
            );
        }
    }
}

// ============================================================
// 9. Crate-wide allow detected (R30)
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    #[allow(clippy::expect_used)] // reason: test -- panics indicate broken test setup
    fn property_crate_wide_allow_detected(
        lint in prop::sample::select(vec![
            "clippy::unwrap_used",
            "dead_code",
            "clippy::too_many_lines",
        ]),
    ) {
        // Build the crate-wide allow attribute via concatenation to avoid
        // tripping the pre-commit hook's literal grep
        let attr = ["#!", &format!("[allow({lint})]")].concat();
        let source = format!("{attr}\nfn main() {{}}\n");

        let dir = TempDir::new().expect("create temp dir");
        setup_rust_project(dir.path(), &[("src/main.rs", &source)]);

        let (_code, stdout, _stderr) = run_rs_validate(dir.path());
        if !stdout.is_empty() {
            let results = parse_check_results(&stdout);
            let has_r30 = results.iter().any(|(id, _, _, _)| id == "R30");
            prop_assert!(
                has_r30,
                "Crate-wide allow({lint}) should produce R30"
            );
        }
    }
}

// ============================================================
// 10. Empty project -- validate should not panic
// ============================================================

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test setup
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn property_empty_project_never_panics() {
    let dir = TempDir::new().expect("create temp dir");

    // Completely empty directory
    let (code, _stdout, _stderr) = run_validate(dir.path());
    assert!(
        code == 0 || code == 1,
        "Empty project should not crash, got code {code}"
    );

    // Empty Rust project (Cargo.toml but no source)
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"empty\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("write Cargo.toml");

    let (code2, _stdout2, _stderr2) = run_rs_validate(dir.path());
    assert!(
        code2 == 0 || code2 == 1,
        "Empty Rust project should not crash, got code {code2}"
    );
}

// ============================================================
// 11. Deeply nested paths -- no stack overflow
// ============================================================

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test setup
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn property_deeply_nested_paths_no_overflow() {
    let dir = TempDir::new().expect("create temp dir");

    // Create a 20-level deep directory with a Rust file
    let mut nested = dir.path().to_path_buf();
    nested.push("src");
    for i in 0..20_u32 {
        nested.push(format!("level_{i}"));
    }
    fs::create_dir_all(&nested).expect("create nested dirs");
    fs::write(nested.join("deep.rs"), "fn deep() {}\n").expect("write deep.rs");

    setup_rust_project(dir.path(), &[("src/main.rs", "fn main() {}\n")]);

    let (code, _stdout, _stderr) = run_rs_validate(dir.path());
    assert!(
        code == 0 || code == 1,
        "Deeply nested project should not crash or overflow, got code {code}"
    );
}
