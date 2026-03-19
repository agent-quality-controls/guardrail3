use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

use super::eslint_rule_infra::{RuleDef, check_eslint_rule, check_eslint_rule_presence};

pub fn check_eslint_config(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let eslint_path = path.join("eslint.config.mjs");
    if !eslint_path.exists() {
        results.push(CheckResult {
            id: "T1".to_owned(),
            severity: Severity::Error,
            title: "ESLint config `eslint.config.mjs` not found".to_owned(),
            message: "ESLint enforces code quality rules (no-unused-vars, naming conventions, import order, \
                     type safety). Without it, no static analysis runs on TypeScript code. \
                     Run `guardrail3 ts generate` to create it, or create `eslint.config.mjs` manually \
                     with the flat config format.".to_owned(),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    results.push(
        CheckResult {
            id: "T1".to_owned(),
            severity: Severity::Info,
            title: "ESLint config exists".to_owned(),
            message: "ESLint flat config `eslint.config.mjs` found at project root.".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );

    let Some(content) = fs.read_file(&eslint_path) else {
        return;
    };

    check_eslint_value_rules(&content, &eslint_path, results);
    check_boundary_enforcement(&content, &eslint_path, results);
    check_eslint_presets(&content, &eslint_path, results);
    check_relaxed_rules(&content, &eslint_path, results);
    check_file_overrides(&content, &eslint_path, results);
    check_rule_presence_t40_t48(&content, &eslint_path, results);
    check_all_eslint_rules(&content, &eslint_path, results);
    check_test_relaxations(&content, &eslint_path, results);
    check_route_wrappers(&content, &eslint_path, results);
    check_process_env_ban(&content, &eslint_path, results);
}

/// T2-T5: `ESLint` rules with expected values.
fn check_eslint_value_rules(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    check_eslint_rule(
        content,
        eslint_path,
        "T2",
        "max-lines",
        Some("400"),
        Severity::Error,
        results,
    );
    check_eslint_rule(
        content,
        eslint_path,
        "T3",
        "max-lines-per-function",
        Some("100"),
        Severity::Error,
        results,
    );
    check_eslint_rule(
        content,
        eslint_path,
        "T4",
        "complexity",
        Some("25"),
        Severity::Error,
        results,
    );
    check_eslint_rule(
        content,
        eslint_path,
        "T5",
        "no-restricted-imports",
        None,
        Severity::Error,
        results,
    );
}

/// T6: Boundary enforcement (boundaries or eslint-plugin-boundaries).
fn check_boundary_enforcement(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    if content.contains("boundaries") || content.contains("eslint-plugin-boundaries") {
        results.push(CheckResult {
            id: "T6".to_owned(),
            severity: Severity::Info,
            title: "Import boundary enforcement configured".to_owned(),
            message: "`eslint-plugin-boundaries` found in config. This enforces hexagonal architecture \
                     import rules — domain cannot import adapters, ports cannot import application, etc."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T6".to_owned(),
            severity: Severity::Warn,
            title: "No import boundary enforcement".to_owned(),
            message: "No `eslint-plugin-boundaries` found in ESLint config. Without boundary enforcement, \
                     domain code can accidentally import from adapters, creating tight coupling that makes \
                     the codebase harder to test and refactor. Install `eslint-plugin-boundaries` and configure \
                     zone definitions in `eslint.config.mjs`."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T-ESLP-13, T-ESLP-14: `ESLint` tseslint preset presence checks.
fn check_eslint_presets(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    if content.contains("strictTypeChecked") {
        results.push(
            CheckResult {
                id: "T-ESLP-13".to_owned(),
                severity: Severity::Info,
                title: "ESLint strictTypeChecked preset configured".to_owned(),
                message:
                    "`tseslint.configs.strictTypeChecked` found in ESLint config. This preset \
                         provides 53+ type-aware lint rules."
                        .to_owned(),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "T-ESLP-13".to_owned(),
            severity: Severity::Error,
            title: "ESLint strictTypeChecked preset missing".to_owned(),
            message:
                "ESLint config must include `tseslint.configs.strictTypeChecked` — this preset \
                     provides 53+ type-aware lint rules."
                    .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    if content.contains("stylisticTypeChecked") {
        results.push(
            CheckResult {
                id: "T-ESLP-14".to_owned(),
                severity: Severity::Info,
                title: "ESLint stylisticTypeChecked preset configured".to_owned(),
                message:
                    "`tseslint.configs.stylisticTypeChecked` found in ESLint config. This preset \
                         provides 18+ code style rules."
                        .to_owned(),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "T-ESLP-14".to_owned(),
            severity: Severity::Error,
            title: "ESLint stylisticTypeChecked preset missing".to_owned(),
            message:
                "ESLint config must include `tseslint.configs.stylisticTypeChecked` — this preset \
                     provides 18+ code style rules."
                    .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T7: Lines containing "off" or "warn" — Info inventory.
fn check_relaxed_rules(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if (trimmed.contains("\"off\"")
            || trimmed.contains("'off'")
            || trimmed.contains("\"warn\"")
            || trimmed.contains("'warn'"))
            && !trimmed.starts_with("//")
            && !trimmed.starts_with('*')
        {
            results.push(CheckResult {
                id: "T7".to_owned(),
                severity: Severity::Info,
                title: "ESLint rule relaxed to off/warn".to_owned(),
                message: format!(
                    "Rule set to `off` or `warn`: `{trimmed}`. Rules turned off disable protection entirely; \
                     rules set to `warn` allow the build to pass with violations. Review whether this relaxation \
                     is justified and add `// EXCEPTION: <reason>` if intentional."
                ),
                file: Some(eslint_path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
                inventory: false,
            }.as_inventory());
        }
    }
}

/// T8: File-specific overrides.
fn check_file_overrides(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.contains("files:") || trimmed.contains("files =") {
            results.push(CheckResult {
                id: "T8".to_owned(),
                severity: Severity::Info,
                title: "File-specific ESLint override".to_owned(),
                message: format!(
                    "File-scoped rule override: `{trimmed}`. File overrides apply different rules to specific \
                     file patterns (e.g., relaxed rules for test files). Verify the scope is narrow and justified."
                ),
                file: Some(eslint_path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
                inventory: false,
            }.as_inventory());
        }
    }
}

/// T40-T48: `ESLint` rule presence checks.
fn check_rule_presence_t40_t48(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    check_eslint_rule_presence(
        content,
        eslint_path,
        "T40",
        "no-floating-promises",
        Severity::Error,
        results,
    );
    check_eslint_rule_presence(
        content,
        eslint_path,
        "T41",
        "no-explicit-any",
        Severity::Error,
        results,
    );
    check_eslint_rule_presence(
        content,
        eslint_path,
        "T42",
        "no-console",
        Severity::Error,
        results,
    );
    check_eslint_rule_presence(
        content,
        eslint_path,
        "T43",
        "eqeqeq",
        Severity::Error,
        results,
    );
    check_eslint_rule_presence(
        content,
        eslint_path,
        "T44",
        "no-restricted-globals",
        Severity::Error,
        results,
    );
    check_eslint_rule_presence(
        content,
        eslint_path,
        "T45",
        "no-cycle",
        Severity::Error,
        results,
    );
    check_eslint_rule_presence(
        content,
        eslint_path,
        "T46",
        "max-dependencies",
        Severity::Error,
        results,
    );
    check_eslint_rule_presence(
        content,
        eslint_path,
        "T47",
        "explicit-function-return-type",
        Severity::Error,
        results,
    );
    check_eslint_rule_presence(
        content,
        eslint_path,
        "T48",
        "strict-boolean-expressions",
        Severity::Error,
        results,
    );
}

/// T49: Test file relaxations.
fn check_test_relaxations(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if (trimmed.contains("test") || trimmed.contains("spec"))
            && (trimmed.contains("files") || trimmed.contains("overrides"))
        {
            results.push(CheckResult {
                id: "T49".to_owned(),
                severity: Severity::Info,
                title: "Test file ESLint relaxation".to_owned(),
                message: format!(
                    "Test-specific rule override: `{trimmed}`. Test files often need relaxed rules \
                     (e.g., no-explicit-any for mocks, max-lines for integration tests). \
                     Verify relaxations are scoped only to test files."
                ),
                file: Some(eslint_path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
                inventory: false,
            }.as_inventory());
        }
    }
}

/// T50: Route wrapper enforcement.
fn check_route_wrappers(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    if content.contains("withBody") || content.contains("withRoute") {
        results.push(
            CheckResult {
                id: "T50".to_owned(),
                severity: Severity::Info,
                title: "Route wrapper enforcement configured".to_owned(),
                message:
                    "`withBody`/`withRoute` patterns found in ESLint config. Route wrappers ensure \
                     all API routes go through validation and error handling middleware."
                        .to_owned(),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "T50".to_owned(),
            severity: Severity::Warn,
            title: "No route wrapper enforcement in ESLint".to_owned(),
            message: "No `withBody`/`withRoute` patterns found in ESLint config. Route wrappers ensure \
                     all API endpoints validate input and handle errors consistently. Add restricted import \
                     rules that require route handlers to use wrapper functions."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T51: process.env ban.
fn check_process_env_ban(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    if content.contains("process.env") {
        results.push(CheckResult {
            id: "T51".to_owned(),
            severity: Severity::Info,
            title: "`process.env` restriction configured in ESLint".to_owned(),
            message: "`process.env` ban found in ESLint config. This forces environment variable access \
                     through a centralized env module, making configuration auditable and validated."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T51".to_owned(),
            severity: Severity::Error,
            title: "No `process.env` restriction in ESLint".to_owned(),
            message: "No `process.env` restriction found in ESLint config. Without this, any file can read \
                     environment variables directly, scattering configuration across the codebase and making it \
                     impossible to audit what config a service needs. Add a `no-restricted-globals` or \
                     `no-restricted-properties` rule banning `process.env` in `eslint.config.mjs`."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// Check all expected `ESLint` rules from the template.
/// Each rule is checked for presence (`content.contains(rule_name)`).
fn check_all_eslint_rules(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    // (check_id, rule_name, severity_if_missing)
    let rules: &[RuleDef] = &[
        ("T60", "no-misused-promises", Severity::Error),
        ("T61", "await-thenable", Severity::Error),
        ("T62", "consistent-type-imports", Severity::Error),
        ("T63", "no-non-null-assertion", Severity::Error),
        ("T64", "switch-exhaustiveness-check", Severity::Error),
        ("T65", "no-unused-vars", Severity::Error),
        ("T66", "require-await", Severity::Error),
        ("T67", "no-param-reassign", Severity::Error),
        ("T68", "no-unsafe-assignment", Severity::Error),
        ("T69", "no-unsafe-member-access", Severity::Error),
        ("T70", "no-unsafe-call", Severity::Error),
        ("T71", "no-unsafe-return", Severity::Error),
        ("T72", "no-unsafe-argument", Severity::Error),
        ("T73", "explicit-module-boundary-types", Severity::Error),
        ("T74", "promise-function-async", Severity::Error),
        ("T75", "consistent-type-exports", Severity::Error),
        ("T76", "consistent-type-definitions", Severity::Error),
        ("T77", "no-unnecessary-condition", Severity::Error),
        ("T78", "prefer-nullish-coalescing", Severity::Error),
        ("T79", "prefer-optional-chain", Severity::Error),
        ("T80", "no-deprecated", Severity::Error),
        ("T81", "restrict-template-expressions", Severity::Error),
        ("T82", "no-throw-literal", Severity::Error),
        ("T83", "no-empty", Severity::Error),
    ];

    for (id, rule_name, severity) in rules {
        check_eslint_rule_presence(content, eslint_path, id, rule_name, *severity, results);
    }
}
