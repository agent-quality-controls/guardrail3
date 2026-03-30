mod cargo_lints_support;

use std::path::{Path, PathBuf};

use self::cargo_lints_support::{
    EXPECTED_CLIPPY_ALLOW, EXPECTED_CLIPPY_DENY, EXPECTED_CLIPPY_GROUPS, EXPECTED_RUST_LINTS,
    LintCheck, emit_expected_allow_inventory, emit_lint_correct, emit_lint_wrong, get_lint_level,
    get_lint_priority,
};
use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

pub fn check(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    cargo_tomls: &[PathBuf],
) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // Find the workspace root Cargo.toml from crawler data
    let root_cargo = cargo_tomls
        .iter()
        .find(|p| p.parent() == Some(workspace_root));

    let Some(cargo_path) = root_cargo else {
        return results;
    };

    let content = match fs.read_file_err(cargo_path) {
        Ok(content) => content,
        Err(e) => {
            results.push(CheckResult::from_parts(
    "R26".to_owned(),
    Severity::Error,
    "Cargo.toml unreadable".to_owned(),
    format!("Failed to read: {e}"),
    Some(cargo_path.display().to_string()),
    None,
    false,
            ));
            return results;
        }
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "R26".to_owned(),
                severity: Severity::Error,
                title: "Cargo.toml parse error".to_owned(),
                message: format!("Invalid TOML: {e}"),
                file: Some(cargo_path.display().to_string()),
                line: None,
                inventory: false,
            });
            return results;
        }
    };

    // Check [workspace.lints.rust]
    let rust_lints = table
        .get("workspace")
        .and_then(|w| w.get("lints"))
        .and_then(|l| l.get("rust"));

    check_rust_lints(rust_lints, cargo_path, &mut results);

    // Check [workspace.lints.clippy]
    let clippy_lints = table
        .get("workspace")
        .and_then(|w| w.get("lints"))
        .and_then(|l| l.get("clippy"));

    check_clippy_lints(clippy_lints, cargo_path, &mut results);

    results,
)

fn check_rust_lints(
    rust_lints: Option<&toml::Value>,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let Some(lints) = rust_lints else {
        results.push(CheckResult::from_parts(
    "R26".to_owned(),
    Severity::Error,
    "[workspace.lints.rust] missing".to_owned(),
    "No Rust lint configuration in workspace".to_owned(),
    Some(file_path.display().to_string()),
    None,
    false,
        ));
        return;
    };

    for exp in EXPECTED_RUST_LINTS {
        check_lint_level(
            lints,
            exp.name,
            exp.expected_level,
            exp.priority,
            "R26",
            "R26",
            file_path,
            Some("[workspace.lints.rust]"),
            results,
        );
    },
)

fn check_clippy_lints(
    clippy_lints: Option<&toml::Value>,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let Some(lints) = clippy_lints else {
        results.push(CheckResult::from_parts(
    "R27".to_owned(),
    Severity::Error,
    "[workspace.lints.clippy] missing".to_owned(),
    "No Clippy lint configuration in workspace".to_owned(),
    Some(file_path.display().to_string()),
    None,
    false,
        ));
        return;
    };

    // Check groups — missing = R26 (completeness), wrong level = R27 (relaxed)
    for exp in EXPECTED_CLIPPY_GROUPS {
        check_lint_level(
            lints,
            exp.name,
            exp.expected_level,
            exp.priority,
            "R26",
            "R27",
            file_path,
            Some("[workspace.lints.clippy]"),
            results,
        );
    }

    // Check specific deny lints — missing = R26 (completeness), wrong level = R27 (relaxed)
    for lint_name in EXPECTED_CLIPPY_DENY {
        check_lint_level(
            lints,
            lint_name,
            "deny",
            None,
            "R26",
            "R27",
            file_path,
            Some("[workspace.lints.clippy]"),
            results,
        );
    }

    for lint_name in EXPECTED_CLIPPY_ALLOW {
        emit_expected_allow_inventory(lints, lint_name, file_path, results);
    },
)

pub fn check_workspace_inheritance(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    member_dirs: &[String],
) -> Vec<CheckResult> {
    let mut results = Vec::new();

    for member in member_dirs {
        let crate_cargo = workspace_root.join(member).join("Cargo.toml");
        if !crate_cargo.exists() {
            continue;
        }

        let Some(content) = fs.read_file(&crate_cargo) else {
            results.push(CheckResult::from_parts(
    "R29".to_owned(),
    Severity::Error,
    "Crate Cargo.toml unreadable".to_owned(),
    format!("{member}: failed to read Cargo.toml for lint inheritance check"),
    Some(crate_cargo.display().to_string()),
    None,
    false,
            ));
            continue;
        };

        let table: toml::Value = match content.parse() {
            Ok(v) => v,
            Err(e) => {
                results.push(CheckResult {
                    id: "R29".to_owned(),
                    severity: Severity::Error,
                    title: "Crate Cargo.toml parse error".to_owned(),
                    message: format!("{member}: invalid TOML in Cargo.toml: {e}"),
                    file: Some(crate_cargo.display().to_string()),
                    line: None,
                    inventory: false,
                });
                continue;
            }
        };

        let workspace_true = table
            .get("lints")
            .and_then(|l| l.get("workspace"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);

        if workspace_true {
            results.push(CheckResult {
                id: "R29".to_owned(),
                severity: Severity::Info,
                title: "Workspace lints inherited".to_owned(),
                message: format!("{member}: [lints] workspace = true. This crate inherits all lint rules from the workspace Cargo.toml, ensuring consistent enforcement. No action needed."),
                file: Some(crate_cargo.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        } else {
            results.push(CheckResult {
                id: "R29".to_owned(),
                severity: Severity::Error,
                title: "Workspace lints not inherited".to_owned(),
                message: format!("{member}: missing `[lints] workspace = true` in Cargo.toml. Without this, the crate does not inherit workspace lint rules, meaning clippy/rustc lints are not enforced consistently. Add `[lints]\nworkspace = true` to this crate's Cargo.toml."),
                file: Some(crate_cargo.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    results,
)

fn check_lint_level(
    lints: &toml::Value,
    name: &str,
    expected_level: &str,
    expected_priority: Option<i64>,
    check_id_missing: &str,
    check_id_wrong: &str,
    file_path: &Path,
    section_hint: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let ctx = LintCheck {
        lints,
        name,
        expected_level,
        expected_priority,
        check_id_missing,
        check_id_wrong,
        file_path,
        section_hint,
    };
    let level = get_lint_level(lints, name);

    match level.as_deref() {
        Some(l) if l == expected_level => {
            emit_lint_correct(&ctx, results);
        }
        Some("forbid") if expected_level == "deny" => {
            results.push(
                CheckResult::from_parts(
                    check_id_missing.to_owned(),
                    Severity::Info,
                    format!("{name} stricter than expected"),
                    format!("{name} = \"forbid\" (expected \"{expected_level}\")"),
                    Some(file_path.display().to_string()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
        Some(l) => {
            emit_lint_wrong(name, expected_level, l, check_id_wrong, file_path, results);
        }
        None => {
            let section_msg = ctx
                .section_hint
                .map_or_else(String::new, |section| format!(" in {section}"));
            results.push(CheckResult::from_parts(
    ctx.check_id_missing.to_owned(),
    Severity::Error,
    format!("{} missing", ctx.name),
    format!(
                    "Expected {} = \"{}\"{section_msg}",
                    ctx.name, ctx.expected_level
                ),
    Some(ctx.file_path.display().to_string()),
    None,
    false,
            ));
        }
    },
)
