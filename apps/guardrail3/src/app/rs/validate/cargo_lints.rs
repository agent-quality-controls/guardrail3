use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

struct LintExpectation {
    name: &'static str,
    expected_level: &'static str,
    priority: Option<i64>,
}

const EXPECTED_RUST_LINTS: &[LintExpectation] = &[
    LintExpectation {
        name: "warnings",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unsafe_code",
        expected_level: "forbid",
        priority: None,
    },
    LintExpectation {
        name: "dead_code",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unused_results",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unused_crate_dependencies",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "missing_debug_implementations",
        expected_level: "warn",
        priority: None,
    },
];

const EXPECTED_CLIPPY_GROUPS: &[LintExpectation] = &[
    LintExpectation {
        name: "all",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "pedantic",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "cargo",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "nursery",
        expected_level: "deny",
        priority: Some(-1),
    },
];

const EXPECTED_CLIPPY_DENY: &[&str] = &[
    "unwrap_used",
    "expect_used",
    "panic",
    "unimplemented",
    "todo",
    "dbg_macro",
    "print_stdout",
    "print_stderr",
    "disallowed_methods",
    "disallowed_types",
    "indexing_slicing",
    "string_slice",
    "arithmetic_side_effects",
    "shadow_unrelated",
    "missing_assert_message",
    "partial_pub_fields",
    "str_to_string",
    "implicit_clone",
    "as_conversions",
    "float_cmp",
    "lossy_float_literal",
    "wildcard_enum_match_arm",
    "rest_pat_in_fully_bound_structs",
    "large_stack_arrays",
    "needless_pass_by_value",
    "redundant_else",
    "large_futures",
    "semicolon_if_nothing_returned",
    "redundant_closure_for_method_calls",
    "map_unwrap_or",
    "verbose_file_reads",
];

const EXPECTED_CLIPPY_ALLOW: &[&str] = &[
    "missing_docs_in_private_items",
    "module_name_repetitions",
    "must_use_candidate",
    "option_if_let_else",
    "empty_line_after_doc_comments",
    "single_match_else",
    "ref_option_ref",
    "trivially_copy_pass_by_ref",
    "multiple_crate_versions",
];

pub fn check(fs: &dyn FileSystem, workspace_root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let cargo_path = workspace_root.join("Cargo.toml");

    if !cargo_path.exists() {
        results.push(CheckResult {
            id: "R26".to_owned(),
            severity: Severity::Error,
            title: "Cargo.toml missing".to_owned(),
            message: "Cannot check workspace lints".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
            inventory: false,
        });
        return results;
    }

    let content = match fs.read_file_err(&cargo_path) {
        Ok(content) => content,
        Err(e) => {
            results.push(CheckResult {
                id: "R26".to_owned(),
                severity: Severity::Error,
                title: "Cargo.toml unreadable".to_owned(),
                message: format!("Failed to read: {e}"),
                file: Some(cargo_path.display().to_string()),
                line: None,
                inventory: false,
            });
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

    check_rust_lints(rust_lints, &cargo_path, &mut results);

    // Check [workspace.lints.clippy]
    let clippy_lints = table
        .get("workspace")
        .and_then(|w| w.get("lints"))
        .and_then(|l| l.get("clippy"));

    check_clippy_lints(clippy_lints, &cargo_path, &mut results);

    results
}

fn check_rust_lints(
    rust_lints: Option<&toml::Value>,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let Some(lints) = rust_lints else {
        results.push(CheckResult {
            id: "R26".to_owned(),
            severity: Severity::Error,
            title: "[workspace.lints.rust] missing".to_owned(),
            message: "No Rust lint configuration in workspace".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
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
    }
}

#[allow(clippy::too_many_lines)] // reason: lint check function covers all expected workspace lints sequentially
fn check_clippy_lints(
    clippy_lints: Option<&toml::Value>,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let Some(lints) = clippy_lints else {
        results.push(CheckResult {
            id: "R27".to_owned(),
            severity: Severity::Error,
            title: "[workspace.lints.clippy] missing".to_owned(),
            message: "No Clippy lint configuration in workspace".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
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

    // R28: Check allows (report as info — extra allows inventory)
    for lint_name in EXPECTED_CLIPPY_ALLOW {
        let level = get_lint_level(lints, lint_name);
        match level.as_deref() {
            Some("allow") => {
                results.push(CheckResult {
                    id: "R28".to_owned(),
                    severity: Severity::Info,
                    title: format!("Allow deviation: {lint_name}"),
                    message: format!("Clippy lint `{lint_name}` is set to `allow` — intentionally disabled because it produces too many false positives or conflicts with project style. Approved deviation, no action needed."),
                    file: Some(file_path.display().to_string()),
                    line: None,
                    inventory: false,
                }.as_inventory());
            }
            Some(other) => {
                results.push(CheckResult {
                    id: "R28".to_owned(),
                    severity: Severity::Info,
                    title: format!("Expected allow: {lint_name}"),
                    message: format!("Clippy lint `{lint_name}` = \"{other}\" (expected \"allow\"). This lint is typically allowed but this project enforces it more strictly. Informational, no action needed."),
                    file: Some(file_path.display().to_string()),
                    line: None,
                    inventory: false,
                }.as_inventory());
            }
            None => {
                results.push(CheckResult {
                    id: "R28".to_owned(),
                    severity: Severity::Info,
                    title: format!("Expected allow missing: {lint_name}"),
                    message: format!("Clippy lint `{lint_name}` is not configured (expected \"allow\"). This lint is typically noisy and allowed, but omission means it defaults to the group level (deny). Consider adding `{lint_name} = \"allow\"` if it produces false positives."),
                    file: Some(file_path.display().to_string()),
                    line: None,
                    inventory: false,
                }.as_inventory());
            }
        }
    }
}

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
            continue;
        };

        let table: toml::Value = match content.parse() {
            Ok(v) => v,
            Err(_) => continue,
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

    results
}

fn get_lint_level(lints: &toml::Value, name: &str) -> Option<String> {
    match lints.get(name) {
        Some(toml::Value::String(s)) => Some(s.clone()),
        Some(toml::Value::Table(t)) => t
            .get("level")
            .and_then(|l| l.as_str())
            .map(std::borrow::ToOwned::to_owned),
        _ => None,
    }
}

fn get_lint_priority(lints: &toml::Value, name: &str) -> Option<i64> {
    match lints.get(name) {
        Some(toml::Value::Table(t)) => t.get("priority").and_then(toml::Value::as_integer),
        _ => None,
    }
}

struct LintCheck<'a> {
    lints: &'a toml::Value,
    name: &'a str,
    expected_level: &'a str,
    expected_priority: Option<i64>,
    check_id_missing: &'a str,
    check_id_wrong: &'a str,
    file_path: &'a Path,
}

#[allow(clippy::too_many_arguments)] // reason: lint validation requires all context params
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
    };
    let level = get_lint_level(lints, name);

    match level.as_deref() {
        Some(l) if l == expected_level => {
            emit_lint_correct(&ctx, results);
        }
        Some("forbid") if expected_level == "deny" => {
            results.push(
                CheckResult {
                    id: check_id_missing.to_owned(),
                    severity: Severity::Info,
                    title: format!("{name} stricter than expected"),
                    message: format!("{name} = \"forbid\" (expected \"{expected_level}\")"),
                    file: Some(file_path.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        }
        Some(l) => {
            emit_lint_wrong(name, expected_level, l, check_id_wrong, file_path, results);
        }
        None => {
            let section_msg = section_hint.map_or_else(String::new, |s| format!(" in {s}"));
            results.push(CheckResult {
                id: check_id_missing.to_owned(),
                severity: Severity::Error,
                title: format!("{name} missing"),
                message: format!("Expected {name} = \"{expected_level}\"{section_msg}"),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

fn emit_lint_correct(ctx: &LintCheck<'_>, results: &mut Vec<CheckResult>) {
    if let Some(exp_pri) = ctx.expected_priority {
        let actual_pri = get_lint_priority(ctx.lints, ctx.name);
        if actual_pri == Some(exp_pri) {
            results.push(
                CheckResult {
                    id: ctx.check_id_missing.to_owned(),
                    severity: Severity::Info,
                    title: format!("{} correct", ctx.name),
                    message: format!("{} = {} (priority {exp_pri})", ctx.name, ctx.expected_level),
                    file: Some(ctx.file_path.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        } else {
            results.push(CheckResult {
                id: ctx.check_id_wrong.to_owned(),
                severity: Severity::Warn,
                title: format!("{} priority wrong", ctx.name),
                message: format!(
                    "Expected priority {exp_pri}, got {}",
                    actual_pri.map_or_else(|| "none".to_owned(), |p| p.to_string())
                ),
                file: Some(ctx.file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    } else {
        results.push(
            CheckResult {
                id: ctx.check_id_missing.to_owned(),
                severity: Severity::Info,
                title: format!("{} correct", ctx.name),
                message: format!("{} = {}", ctx.name, ctx.expected_level),
                file: Some(ctx.file_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

fn emit_lint_wrong(
    name: &str,
    expected_level: &str,
    actual_level: &str,
    check_id_wrong: &str,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let is_weakened = matches!(
        (expected_level, actual_level),
        ("deny" | "forbid", "warn" | "allow") | ("forbid", "deny")
    );
    results.push(CheckResult {
        id: check_id_wrong.to_owned(),
        severity: if is_weakened {
            Severity::Error
        } else {
            Severity::Warn
        },
        title: format!("{name} wrong level"),
        message: format!("Expected \"{expected_level}\", got \"{actual_level}\""),
        file: Some(file_path.display().to_string()),
        line: None,
        inventory: false,
    });
}
