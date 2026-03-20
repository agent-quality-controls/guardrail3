//! Adversarial integration tests for guardrail3 check categories feature.
//!
//! Each test creates a minimal Rust project in a temp directory (with or without
//! guardrail3.toml), runs `guardrail3 rs validate --format json`, and asserts
//! that the correct check categories are present or absent based on config/CLI flags.
use garde as _;

// Suppress unused crate dependency warnings for crates used only by the main binary
use clap as _;
use colored as _;
use glob as _;
use guardrail3 as _;
use ignore as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use serde as _;
use std::process::Command;
use syn as _;
use toml as _;
use toml_edit as _;
use tree_sitter as _;
use tree_sitter_javascript as _;
use tree_sitter_typescript as _;
use walkdir as _;

const MINIMAL_CARGO_TOML: &str = r#"[package]
name = "category-test"
version = "0.1.0"
edition = "2024"

[lints]
workspace = true

[workspace.lints.rust]
unsafe_code = "forbid"
"#;

const MINIMAL_LIB_RS: &str = r#"pub fn hello() -> &'static str {
    "hello"
}
"#;

/// Create a temp project with a minimal Cargo.toml + src/lib.rs and optionally a guardrail3.toml.
/// Returns the temp dir (held alive by the caller).
#[allow(clippy::disallowed_methods)] // reason: fs operations needed to set up temp project
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn setup_project(guardrail3_toml: Option<&str>) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let src_dir = tmp.path().join("src");
    std::fs::create_dir_all(&src_dir).expect("create src/");
    std::fs::write(tmp.path().join("Cargo.toml"), MINIMAL_CARGO_TOML).expect("write Cargo.toml");
    std::fs::write(src_dir.join("lib.rs"), MINIMAL_LIB_RS).expect("write lib.rs");

    if let Some(toml_content) = guardrail3_toml {
        std::fs::write(tmp.path().join("guardrail3.toml"), toml_content)
            .expect("write guardrail3.toml");
    }

    tmp
}

/// Run guardrail3 rs validate --format json on the given path with extra CLI args.
/// Returns the JSON stdout as a string.
#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn run_validate(path: &std::path::Path, extra_args: &[&str]) -> String {
    let mut args = vec!["rs", "validate", "--format", "json"];
    args.extend_from_slice(extra_args);
    args.push(path.to_str().expect("path"));

    let out = Command::new(env!("CARGO_BIN_EXE_guardrail3"))
        .args(&args)
        .output()
        .expect("failed to run guardrail3");

    String::from_utf8_lossy(&out.stdout).to_string()
}

/// Collect all check IDs from the JSON output into a Vec.
#[allow(clippy::expect_used, clippy::indexing_slicing)] // reason: test helper — JSON parsing for assertion
fn collect_check_ids(json_output: &str) -> Vec<String> {
    #[allow(clippy::disallowed_methods)] // reason: test helper — JSON parsing of guardrail3 output
    let parsed: serde_json::Value =
        serde_json::from_str(json_output).expect("guardrail3 output should be valid JSON");

    let sections = parsed["sections"]
        .as_array()
        .expect("sections should be array");

    let mut ids = Vec::new();
    for section in sections {
        let results = section["results"].as_array().expect("results array");
        for result in results {
            if let Some(id) = result["id"].as_str() {
                ids.push(id.to_owned());
            }
        }
    }
    ids
}

/// Assert that at least one check ID matching the prefix exists in the output.
#[allow(clippy::expect_used)] // reason: test assertion helper
fn assert_has_check_prefix(ids: &[String], prefix: &str, json_output: &str) {
    let found = ids.iter().any(|id| id.starts_with(prefix));
    assert!(
        found,
        "Expected at least one check starting with '{prefix}' in output.\nCheck IDs found: {ids:?}\nFull output:\n{json_output}"
    );
}

/// Assert that NO check ID matching the prefix exists in the output.
fn assert_no_check_prefix(ids: &[String], prefix: &str, json_output: &str) {
    let matching: Vec<_> = ids.iter().filter(|id| id.starts_with(prefix)).collect();
    assert!(
        matching.is_empty(),
        "Did NOT expect any check starting with '{prefix}', but found: {matching:?}\nFull output:\n{json_output}"
    );
}

/// Assert a specific check ID exists in the output.
#[allow(clippy::expect_used)] // reason: test assertion helper
fn assert_has_check(ids: &[String], check_id: &str, json_output: &str) {
    let found = ids.iter().any(|id| id == check_id);
    assert!(
        found,
        "Expected check '{check_id}' in output.\nCheck IDs found: {ids:?}\nFull output:\n{json_output}"
    );
}

/// Assert a specific check ID does NOT exist in the output.
#[allow(dead_code)] // reason: test helper used by category tests below
fn assert_no_check(ids: &[String], check_id: &str, json_output: &str) {
    assert!(
        !ids.iter().any(|id| id == check_id),
        "Did NOT expect check '{check_id}', but it was present.\nFull output:\n{json_output}"
    );
}

// ============================================================
// Test 1: Default behavior — no config, no CLI flags
// Core checks present, architecture and garde absent
// ============================================================

#[test]
fn categories_default_no_config_has_core_checks() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Core checks should be present (R1 = clippy.toml, R26 = workspace lints)
    assert_has_check(&ids, "R1", &output);
    assert_has_check(&ids, "R26", &output);
}

#[test]
fn categories_default_no_config_has_architecture_checks() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Architecture checks SHOULD appear by default (all categories on)
    assert_has_check_prefix(&ids, "R-ARCH-", &output);
}

#[test]
fn categories_default_no_config_has_garde_checks() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Garde checks SHOULD appear by default (all categories on)
    assert_has_check_prefix(&ids, "R-GARDE-", &output);
}

#[test]
fn categories_config_disables_architecture() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
architecture = false
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Architecture checks should NOT appear when config disables them
    assert_no_check_prefix(&ids, "R-ARCH-", &output);
}

#[test]
fn categories_config_disables_garde() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
garde = false
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Garde checks should NOT appear when config disables them
    assert_no_check_prefix(&ids, "R-GARDE-", &output);
}

// ============================================================
// Test 2: Config enables architecture category
// ============================================================

#[test]
fn categories_config_enables_architecture() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
architecture = true
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Architecture checks SHOULD appear when config enables them
    assert_has_check_prefix(&ids, "R-ARCH-", &output);
}

// ============================================================
// Test 3: Config enables garde category
// ============================================================

#[test]
fn categories_config_enables_garde() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
garde = true
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Garde checks SHOULD appear when config enables them
    assert_has_check_prefix(&ids, "R-GARDE-", &output);
}

// ============================================================
// Test 4: CLI --garde flag enables garde without config
// ============================================================

#[test]
fn categories_cli_garde_flag_enables_garde() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &["--garde"]);
    let ids = collect_check_ids(&output);

    // Garde checks SHOULD appear when --garde flag is used
    assert_has_check_prefix(&ids, "R-GARDE-", &output);
}

// ============================================================
// Test 5: CLI --architecture flag enables architecture, excludes tests
// ============================================================

#[test]
fn categories_cli_architecture_flag_enables_architecture() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &["--architecture"]);
    let ids = collect_check_ids(&output);

    // Architecture checks SHOULD appear when --architecture flag is used
    assert_has_check_prefix(&ids, "R-ARCH-", &output);
}

#[test]
fn categories_cli_architecture_flag_excludes_tests() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &["--architecture"]);
    let ids = collect_check_ids(&output);

    // Test checks should NOT appear when only --architecture is specified
    // (CLI flags act as a filter — only the requested category runs)
    assert_no_check_prefix(&ids, "R-TEST-", &output);
}

// ============================================================
// Test 6: Tests category defaults to on
// ============================================================

#[test]
fn categories_tests_default_on_without_config() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Test checks SHOULD appear by default (tests category defaults to true)
    assert_has_check_prefix(&ids, "R-TEST-", &output);
}

// ============================================================
// Test 7: Config disables tests category
// ============================================================

#[test]
fn categories_config_disables_tests() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
tests = false
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Test checks should NOT appear when config disables them
    assert_no_check_prefix(&ids, "R-TEST-", &output);
}

// ============================================================
// Additional adversarial tests: edge cases and interactions
// ============================================================

#[test]
fn categories_cli_flag_overrides_config_disabled() {
    // Config disables tests, but CLI --tests flag should re-enable them
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
tests = false
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &["--tests"]);
    let ids = collect_check_ids(&output);

    // CLI --tests flag should enable test checks regardless of config
    assert_has_check_prefix(&ids, "R-TEST-", &output);
}

#[test]
fn categories_cli_garde_does_not_include_architecture() {
    // --garde flag should only enable garde, not architecture
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &["--garde"]);
    let ids = collect_check_ids(&output);

    assert_has_check_prefix(&ids, "R-GARDE-", &output);
    assert_no_check_prefix(&ids, "R-ARCH-", &output);
}

#[test]
fn categories_cli_architecture_does_not_include_garde() {
    // --architecture flag should only enable architecture, not garde
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &["--architecture"]);
    let ids = collect_check_ids(&output);

    assert_has_check_prefix(&ids, "R-ARCH-", &output);
    assert_no_check_prefix(&ids, "R-GARDE-", &output);
}

#[test]
fn categories_config_enables_all_optional() {
    // Config enables all optional categories at once
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
architecture = true
garde = true
tests = true
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // All categories should be present
    assert_has_check_prefix(&ids, "R-ARCH-", &output);
    assert_has_check_prefix(&ids, "R-GARDE-", &output);
    assert_has_check_prefix(&ids, "R-TEST-", &output);
    // Core checks should still be present
    assert_has_check(&ids, "R1", &output);
}

#[test]
fn categories_empty_checks_section_uses_defaults() {
    // Config has [rust.checks] but no fields — should use defaults (all on)
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // All categories on by default
    assert_has_check_prefix(&ids, "R-TEST-", &output);
    assert_has_check_prefix(&ids, "R-ARCH-", &output);
    assert_has_check_prefix(&ids, "R-GARDE-", &output);
}

#[test]
fn categories_config_without_checks_section_uses_defaults() {
    // Config has [rust] but no [rust.checks] — should use defaults (all on)
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // All categories on by default
    assert_has_check_prefix(&ids, "R-TEST-", &output);
    assert_has_check_prefix(&ids, "R-ARCH-", &output);
    assert_has_check_prefix(&ids, "R-GARDE-", &output);
}

// ============================================================
// TypeScript per-app type profile tests
// ============================================================

/// Create a temp TS monorepo with root package.json, per-app dirs, and optional guardrail3.toml.
/// `apps` is a slice of `(name, has_hex_arch)` tuples.
#[allow(clippy::disallowed_methods)] // reason: test helper — creates temp TS project structure
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
#[allow(clippy::type_complexity)] // reason: test helper — tuple slice parameter is clear in context
fn setup_ts_monorepo(config: Option<&str>, apps: &[(&str, bool)]) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");

    // Root package.json
    std::fs::write(
        tmp.path().join("package.json"),
        r#"{"name": "test-monorepo", "private": true}"#,
    )
    .expect("write root package.json");

    // Create each app
    for (name, has_hex_arch) in apps {
        let app_dir = tmp.path().join("apps").join(name);
        let src_dir = app_dir.join("src");
        std::fs::create_dir_all(&src_dir).expect("create app src/");

        // App package.json
        std::fs::write(
            app_dir.join("package.json"),
            format!(r#"{{"name": "{name}", "version": "0.1.0"}}"#),
        )
        .expect("write app package.json");

        // A .ts file so it's detected as TS app
        std::fs::write(src_dir.join("index.ts"), "export const x = 1;").expect("write ts file");

        if *has_hex_arch {
            let modules = src_dir.join("modules");
            std::fs::create_dir_all(modules.join("domain")).expect("create domain/");
            std::fs::create_dir_all(modules.join("application")).expect("create application/");
            std::fs::create_dir_all(modules.join("adapters")).expect("create adapters/");
            std::fs::write(
                modules.join("domain").join("index.ts"),
                "export type X = string;",
            )
            .expect("write domain");
            std::fs::write(
                modules.join("application").join("index.ts"),
                "export const cmd = 1;",
            )
            .expect("write application");
            std::fs::write(
                modules.join("adapters").join("index.ts"),
                "export const y = 1;",
            )
            .expect("write adapters");
        }
    }

    // Write guardrail3.toml if provided
    if let Some(config_content) = config {
        std::fs::write(tmp.path().join("guardrail3.toml"), config_content).expect("write config");
    }

    tmp
}

/// Run guardrail3 ts validate --format json on the given path with extra CLI args.
#[allow(clippy::disallowed_methods)] // reason: test helper — Command::new for binary under test
#[allow(clippy::expect_used)] // reason: test helper
fn run_ts_validate(path: &std::path::Path, extra_args: &[&str]) -> String {
    let mut args = vec!["ts", "validate", "--format", "json"];
    args.extend_from_slice(extra_args);
    args.push(path.to_str().expect("path"));

    let out = Command::new(env!("CARGO_BIN_EXE_guardrail3"))
        .args(&args)
        .output()
        .expect("failed to run guardrail3");

    String::from_utf8_lossy(&out.stdout).to_string()
}

// ============================================================
// Test: Service app type — arch checks fire based on structure
// ============================================================

#[test]
fn ts_app_type_service_gets_arch_checks() {
    // Service app WITH hex arch dirs → T-ARCH-01 should NOT fire (structure exists)
    let config_with_arch = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.api]
type = "service"
"#;
    let tmp = setup_ts_monorepo(Some(config_with_arch), &[("api", true)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Service app has the hex arch dirs, so T-ARCH-01 should NOT fire
    assert_no_check(&ids, "T-ARCH-01", &output);

    // Service app WITHOUT hex arch dirs → T-ARCH-01 SHOULD fire (missing structure)
    let config_no_arch = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.api]
type = "service"
"#;
    let tmp2 = setup_ts_monorepo(Some(config_no_arch), &[("api", false)]);
    let output2 = run_ts_validate(tmp2.path(), &[]);
    let ids2 = collect_check_ids(&output2);

    // Service app missing hex arch dirs — T-ARCH-01 should fire
    assert_has_check(&ids2, "T-ARCH-01", &output2);
}

// ============================================================
// Test: Content app type skips architecture checks
// ============================================================

#[test]
fn ts_app_type_content_skips_arch_checks() {
    // Content app WITHOUT hex arch dirs → T-ARCH-01 should NOT fire
    // (content apps skip architecture checks entirely)
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.landing]
type = "content"
"#;
    let tmp = setup_ts_monorepo(Some(config), &[("landing", false)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Content app — architecture checks should not fire at all
    assert_no_check(&ids, "T-ARCH-01", &output);
}

// ============================================================
// Test: Content app with architecture override enabled
// ============================================================

#[test]
fn ts_app_type_content_override_enables_arch() {
    // Content app with checks.architecture = true → T-ARCH-01 SHOULD fire
    // (per-app override enables arch checks even for content type)
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.landing]
type = "content"

[typescript.apps.landing.checks]
architecture = true
"#;
    let tmp = setup_ts_monorepo(Some(config), &[("landing", false)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Content app with arch override — T-ARCH-01 should fire (missing structure)
    assert_has_check(&ids, "T-ARCH-01", &output);
}

// ============================================================
// Test: Default app type is service when no type configured
// ============================================================

#[test]
fn ts_app_type_default_is_service() {
    // No [typescript.apps] config at all → app should behave as service
    // Service type without hex arch → T-ARCH-01 should fire
    let tmp = setup_ts_monorepo(None, &[("api", false)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Default is service — missing arch should fire T-ARCH-01
    assert_has_check(&ids, "T-ARCH-01", &output);
}

// ============================================================
// Test: Mixed monorepo — service gets arch, content skips arch
// ============================================================

#[test]
fn ts_mixed_monorepo_per_app_types() {
    // Two apps: service (no hex arch → should get T-ARCH-01)
    //           content (no hex arch → should NOT get T-ARCH-01)
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.api]
type = "service"

[typescript.apps.landing]
type = "content"
"#;
    let tmp = setup_ts_monorepo(Some(config), &[("api", false), ("landing", false)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Service app missing arch → T-ARCH-01 should fire (at least once, for api)
    assert_has_check(&ids, "T-ARCH-01", &output);

    // The T-ARCH-01 messages should reference the service app (api), not the content app (landing)
    // Parse the full output to verify which app triggered the check
    #[allow(clippy::disallowed_methods)] // reason: test assertion — JSON parsing
    #[allow(clippy::expect_used)] // reason: test assertion
    #[allow(clippy::indexing_slicing)] // reason: test assertion — JSON access
    {
        let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
        let sections = parsed["sections"].as_array().expect("sections array");

        let arch_results: Vec<_> = sections
            .iter()
            .flat_map(|s| {
                s["results"]
                    .as_array()
                    .expect("results array")
                    .iter()
                    .filter(|r| r["id"].as_str() == Some("T-ARCH-01"))
                    .collect::<Vec<_>>()
            })
            .collect();

        // T-ARCH-01 should reference "api" (service app missing arch)
        let mentions_api = arch_results.iter().any(|r| {
            r["title"].as_str().is_some_and(|t| t.contains("api"))
                || r["message"].as_str().is_some_and(|m| m.contains("api"))
        });
        assert!(
            mentions_api,
            "T-ARCH-01 should fire for the 'api' service app.\nArch results: {arch_results:?}"
        );

        // T-ARCH-01 should NOT reference "landing" (content app skips arch)
        let mentions_landing = arch_results.iter().any(|r| {
            r["title"].as_str().is_some_and(|t| t.contains("landing"))
                || r["message"].as_str().is_some_and(|m| m.contains("landing"))
        });
        assert!(
            !mentions_landing,
            "T-ARCH-01 should NOT fire for the 'landing' content app.\nArch results: {arch_results:?}"
        );
    }
}

// ============================================================
// Adversarial tests: T-ARCH-02 import boundary with per-app types
// ============================================================

/// Helper: write a file with a forbidden domain→adapters import into an app.
#[allow(clippy::disallowed_methods)] // reason: test helper — creates violation fixture file
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn inject_import_violation(tmp: &tempfile::TempDir, app_name: &str) {
    let bad_file = tmp
        .path()
        .join("apps")
        .join(app_name)
        .join("src/modules/domain/bad_import.ts");
    std::fs::write(&bad_file, "import { something } from '../adapters/db';\n")
        .expect("write bad_import.ts");
}

#[test]
fn ts_content_app_import_violation_not_flagged() {
    // Content app with a domain→adapters import violation.
    // T-ARCH-02 should NOT fire because content apps skip architecture checks.
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.landing]
type = "content"
"#;
    let tmp = setup_ts_monorepo(Some(config), &[("landing", true)]);
    inject_import_violation(&tmp, "landing");

    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    assert_no_check(&ids, "T-ARCH-02", &output);
}

#[test]
fn ts_service_app_import_violation_flagged() {
    // Service app with a domain→adapters import violation.
    // T-ARCH-02 SHOULD fire because service apps enforce architecture checks.
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.api]
type = "service"
"#;
    let tmp = setup_ts_monorepo(Some(config), &[("api", true)]);
    inject_import_violation(&tmp, "api");

    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    assert_has_check(&ids, "T-ARCH-02", &output);
}

#[test]
fn ts_library_app_import_violation_not_flagged() {
    // Library app with a domain→adapters import violation.
    // T-ARCH-02 should NOT fire because library apps skip architecture checks.
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.utils]
type = "library"
"#;
    let tmp = setup_ts_monorepo(Some(config), &[("utils", true)]);
    inject_import_violation(&tmp, "utils");

    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    assert_no_check(&ids, "T-ARCH-02", &output);
}

// ============================================================
// Adversarial tests: per-app type profile edge cases
// ============================================================

/// Typo in app type config (`"servce"` instead of `"service"`).
/// `from_str_or_default` treats unknown strings as Service, so arch checks should
/// still fire for a service-like app missing hex arch structure.
#[test]
fn ts_app_config_typo_in_type_defaults_to_service() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.api]
type = "servce"
"#;
    let tmp = setup_ts_monorepo(Some(config), &[("api", false)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Typo in type → defaults to Service → arch checks enabled → T-ARCH-01 fires
    assert_has_check(&ids, "T-ARCH-01", &output);
}

/// Config has `[typescript.apps.wrong-name]` but the actual app dir is `apps/api`.
/// The config entry doesn't match any discovered app — `api` should get default
/// (service) behavior since its name has no config entry.
#[test]
fn ts_app_config_name_mismatch_ignored() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.wrong-name]
type = "content"
"#;
    // Actual app is "api", but config only has "wrong-name"
    let tmp = setup_ts_monorepo(Some(config), &[("api", false)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // "api" has no matching config → defaults to Service → arch checks fire
    assert_has_check(&ids, "T-ARCH-01", &output);
}

/// Config has `[typescript.apps.ghost-app]` for an app that doesn't exist on disk.
/// Discovery is disk-based, so the ghost config should be silently ignored.
/// No crash, no spurious results.
#[test]
fn ts_app_extra_config_no_matching_dir() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.ghost-app]
type = "service"

[typescript.apps.api]
type = "content"
"#;
    // Only "api" exists on disk; "ghost-app" is in config but has no directory
    let tmp = setup_ts_monorepo(Some(config), &[("api", false)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // "api" is content type → arch skipped → T-ARCH-01 should NOT fire
    assert_no_check(&ids, "T-ARCH-01", &output);

    // Should not crash — if we got valid JSON, we're good
    assert!(
        !output.is_empty(),
        "Output should be non-empty valid JSON, not a crash"
    );
}

/// Library type app without hex arch should NOT get T-ARCH-01.
/// Libraries skip architecture checks the same way content does.
#[test]
fn ts_app_library_type_skips_arch() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.utils]
type = "library"
"#;
    let tmp = setup_ts_monorepo(Some(config), &[("utils", false)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Library type → architecture defaults to false → T-ARCH-01 should NOT fire
    assert_no_check(&ids, "T-ARCH-01", &output);
}

/// Config has `type = "Content"` (uppercase C).
/// `from_str_or_default` is case-insensitive, so "Content" correctly matches content type.
#[test]
fn ts_app_type_case_insensitive() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.landing]
type = "Content"
"#;
    let tmp = setup_ts_monorepo(Some(config), &[("landing", false)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // "Content" (capital C) matches content type → architecture skipped → no T-ARCH-01
    assert_no_check(&ids, "T-ARCH-01", &output);
}

/// Service app with `checks.architecture = false` — the per-app override should
/// suppress T-ARCH checks even though service type defaults to architecture=true.
#[test]
fn ts_app_checks_override_false_on_service() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.api]
type = "service"

[typescript.apps.api.checks]
architecture = false
"#;
    let tmp = setup_ts_monorepo(Some(config), &[("api", false)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Service type defaults to architecture=true, but per-app override sets it false
    // → T-ARCH-01 should NOT fire
    assert_no_check(&ids, "T-ARCH-01", &output);
}

/// Three apps — one service (no arch, gets T-ARCH-01), one content (no arch, skips),
/// one library (no arch, skips). Verify only the service app triggers T-ARCH-01.
#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion — JSON access for verifying per-app results
#[allow(clippy::expect_used)] // reason: test assertion — JSON parsing
fn ts_monorepo_three_types_mixed() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.apps.api]
type = "service"

[typescript.apps.landing]
type = "content"

[typescript.apps.utils]
type = "library"
"#;
    let tmp = setup_ts_monorepo(
        Some(config),
        &[("api", false), ("landing", false), ("utils", false)],
    );
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // T-ARCH-01 should fire (for the service app "api")
    assert_has_check(&ids, "T-ARCH-01", &output);

    // Verify T-ARCH-01 only references "api", not "landing" or "utils"
    #[allow(clippy::disallowed_methods)] // reason: test assertion — JSON parsing
    {
        let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON output");
        let sections = parsed["sections"].as_array().expect("sections array");

        let arch_results: Vec<_> = sections
            .iter()
            .flat_map(|s| {
                s["results"]
                    .as_array()
                    .expect("results array")
                    .iter()
                    .filter(|r| r["id"].as_str() == Some("T-ARCH-01"))
                    .collect::<Vec<_>>()
            })
            .collect();

        // Should NOT mention content or library apps
        for result in &arch_results {
            let title = result["title"].as_str().unwrap_or("");
            let message = result["message"].as_str().unwrap_or("");
            let combined = format!("{title} {message}");

            assert!(
                !combined.contains("landing"),
                "T-ARCH-01 should not mention content app 'landing'.\nResult: {result}"
            );
            assert!(
                !combined.contains("utils"),
                "T-ARCH-01 should not mention library app 'utils'.\nResult: {result}"
            );
        }
    }
}

/// Project has package.json at root but no apps/ directory.
/// Should not crash, should produce no T-ARCH results.
#[test]
#[allow(clippy::disallowed_methods)] // reason: test setup — fs operations to create minimal project
#[allow(clippy::expect_used)] // reason: test setup — panics indicate broken test infrastructure
fn ts_app_no_apps_dir_no_crash() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");

    // Root package.json only, no apps/ directory at all
    std::fs::write(
        tmp.path().join("package.json"),
        r#"{"name": "test-project", "private": true}"#,
    )
    .expect("write package.json");

    // A .ts file at the root level so it's detected as a TS project
    let src_dir = tmp.path().join("src");
    std::fs::create_dir_all(&src_dir).expect("create src/");
    std::fs::write(src_dir.join("index.ts"), "export const x = 1;").expect("write ts file");

    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // No apps/ dir means no apps discovered → no T-ARCH-01
    assert_no_check_prefix(&ids, "T-ARCH-", &output);

    // Should not crash — valid JSON output
    assert!(
        !output.is_empty(),
        "Output should be non-empty valid JSON, not a crash"
    );
}

/// Global `[typescript.checks] architecture = false` should override service type defaults.
///
/// The global `categories.architecture` flag gates the entire arch section in
/// `ts::validate::run()` (line 45: `if categories.architecture`). If global is false,
/// `resolve_app_contexts` is never called, so per-app type defaults are irrelevant.
/// This tests that the global setting wins over service type defaults.
#[test]
fn ts_global_architecture_false_overrides_service_type() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.checks]
architecture = false

[typescript.apps.api]
type = "service"
"#;
    // Service app without hex arch — would normally trigger T-ARCH-01
    let tmp = setup_ts_monorepo(Some(config), &[("api", false)]);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Global architecture=false gates the entire arch section → T-ARCH-01 should NOT fire
    // even though the app type is "service" (which defaults to architecture=true).
    // This is the current behavior: global wins because it short-circuits before per-app
    // resolution. Whether this is a bug or feature depends on design intent — but this
    // test documents what actually happens.
    assert_no_check(&ids, "T-ARCH-01", &output);
    assert_no_check_prefix(&ids, "T-ARCH-", &output);
}
