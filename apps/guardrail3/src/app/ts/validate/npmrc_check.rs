use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

type NpmrcExpectation<'a> = (&'a str, &'a str);
type NpmrcSettings = Vec<(String, String)>;

#[allow(clippy::string_slice)] // reason: parsing known ASCII key=value pairs
pub fn check_npmrc(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let npmrc_path = path.join(".npmrc");
    if !npmrc_path.exists() {
        results.push(CheckResult {
            id: "T11".to_owned(),
            severity: Severity::Error,
            title: "`.npmrc` config file not found".to_owned(),
            message: "No `.npmrc` found at project root. The `.npmrc` file configures pnpm behavior — \
                     strict peer dependencies, workspace cycle prevention, supply chain security settings. \
                     Without it, pnpm uses permissive defaults that allow dependency conflicts and security \
                     issues. Create `.npmrc` with the guardrail baseline settings or run `guardrail3 ts generate`."
                .to_owned(),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    results.push(CheckResult {
        id: "T11".to_owned(),
        severity: Severity::Info,
        title: "`.npmrc` config exists".to_owned(),
        message: "pnpm configuration file `.npmrc` found at project root.".to_owned(),
        file: Some(npmrc_path.display().to_string()),
        line: None,
        inventory: false,
    }.as_inventory());

    let Some(content) = fs.read_file(&npmrc_path) else {
        return;
    };

    let settings = parse_npmrc_settings(&content);
    check_expected_settings(&settings, &npmrc_path, results);
    check_extra_settings(&settings, &npmrc_path, results);
}

/// Parse key=value pairs from .npmrc content, skipping comments and blank lines.
#[allow(clippy::string_slice)] // reason: parsing known ASCII key=value pairs
fn parse_npmrc_settings(content: &str) -> NpmrcSettings {
    let mut settings = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }
        if let Some(eq_pos) = trimmed.find('=') {
            #[allow(clippy::string_slice)] // reason: parsing known ASCII key=value pairs
            let key = trimmed[..eq_pos].trim().to_owned();
            let value = trimmed[eq_pos.saturating_add(1)..].trim().to_owned();
            settings.push((key, value));
        }
    }
    settings
}

/// T12/T13: Check each expected .npmrc setting is present with correct value.
fn check_expected_settings(
    settings: &NpmrcSettings,
    npmrc_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let expected: &[NpmrcExpectation<'_>] = &[
        ("strict-peer-dependencies", "true"),
        ("disallow-workspace-cycles", "true"),
        ("save-workspace-protocol", "rolling"),
        ("engine-strict", "true"),
        ("package-manager-strict-version", "true"),
        ("strict-dep-builds", "true"),
        ("verify-deps-before-run", "error"),
        ("minimum-release-age", "1440"),
        ("block-exotic-subdeps", "true"),
        ("trust-policy", "warn"),
        ("public-hoist-pattern", ""),
        ("save-prefix", ""),
        ("shamefully-hoist", "false"),
    ];

    for (key, expected_val) in expected {
        let found = settings.iter().find(|(k, _)| k == key);
        match found {
            Some((_, val)) if val == expected_val => {
                // Correct — no output needed
            }
            Some((_, val)) => {
                // T13: Weaker value
                results.push(CheckResult {
                    id: "T13".to_owned(),
                    severity: Severity::Error,
                    title: format!("`.npmrc` setting `{key}` has wrong value"),
                    message: format!(
                        "`{key}` is set to `{val}` but should be `{expected_val}`. \
                         This setting controls pnpm strictness — a weaker value reduces protection against \
                         dependency conflicts or supply chain issues. Update `.npmrc` to set `{key}={expected_val}`."
                    ),
                    file: Some(npmrc_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "T12".to_owned(),
                    severity: Severity::Error,
                    title: format!("`.npmrc` setting `{key}` missing"),
                    message: format!(
                        "`{key}` not found in `.npmrc`. This setting is required for guardrail compliance. \
                         Add `{key}={expected_val}` to `.npmrc`."
                    ),
                    file: Some(npmrc_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }
}

/// T14: Extra settings not in expected list.
fn check_extra_settings(
    settings: &NpmrcSettings,
    npmrc_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let expected_keys: &[&str] = &[
        "strict-peer-dependencies",
        "disallow-workspace-cycles",
        "save-workspace-protocol",
        "engine-strict",
        "package-manager-strict-version",
        "strict-dep-builds",
        "verify-deps-before-run",
        "minimum-release-age",
        "block-exotic-subdeps",
        "trust-policy",
        "public-hoist-pattern",
        "save-prefix",
        "shamefully-hoist",
    ];

    for (key, val) in settings {
        if !expected_keys.contains(&key.as_str()) {
            results.push(CheckResult {
                id: "T14".to_owned(),
                severity: Severity::Info,
                title: format!("Extra `.npmrc` setting: `{key}`"),
                message: format!(
                    "Non-baseline `.npmrc` setting `{key}={val}`. This setting is not in the guardrail baseline. \
                     Verify it is intentional and document why it's needed."
                ),
                file: Some(npmrc_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        }
    }
}
