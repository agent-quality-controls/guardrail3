use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

type NpmrcExpectation<'a> = (&'a str, &'a str);
type NpmrcSettings = Vec<(String, String)>;

#[allow(clippy::string_slice)] // reason: parsing known ASCII key=value pairs
pub fn check_npmrc(
    fs: &dyn FileSystem,
    npmrcs: &[PathBuf],
    root: &Path,
    results: &mut Vec<CheckResult>,
) {
    if npmrcs.is_empty() {
        results.push(CheckResult::from_parts(
    "T11".to_owned(),
    Severity::Error,
    "`.npmrc` config file not found".to_owned(),
    "No `.npmrc` found at project root. The `.npmrc` file configures pnpm behavior — \
                     strict peer dependencies, workspace cycle prevention, supply chain security settings. \
                     Without it, pnpm uses permissive defaults that allow dependency conflicts and security \
                     issues. Create `.npmrc` with the guardrail baseline settings or run `guardrail3 ts generate`."
                .to_owned(),
    Some(root.display().to_string()),
    None,
    false,
        ));
        return;
    }

    for npmrc_path in npmrcs {
        results.push(
            CheckResult::from_parts(
                "T11".to_owned(),
                Severity::Info,
                "`.npmrc` config exists".to_owned(),
                format!("pnpm configuration file found: `{}`.", npmrc_path.display()),
                Some(npmrc_path.display().to_string()),
                None,
                false,
            )
            .as_inventory(),
        );

        let Some(content) = fs.read_file(npmrc_path) else {
            continue;
        };

        let settings = parse_npmrc_settings(&content);
        check_duplicate_keys(&settings, npmrc_path, results);
        check_expected_settings(&settings, npmrc_path, results);
        check_extra_settings(&settings, npmrc_path, results);
    }
}

/// Parse key=value pairs from .npmrc content, skipping comments and blank lines.
#[allow(clippy::string_slice)] // reason: parsing known ASCII key=value pairs
fn parse_npmrc_settings(content: &str) -> NpmrcSettings {
    let mut settings = Vec::new();
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(content);
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }
        if let Some(eq_pos) = trimmed.find('=') {
            #[allow(clippy::string_slice)] // reason: parsing known ASCII key=value pairs
            let key = trimmed[..eq_pos].trim().to_owned();
            let raw_value = trimmed[eq_pos.saturating_add(1)..].trim();
            let value = raw_value
                .strip_prefix('"')
                .and_then(|v| v.strip_suffix('"'))
                .unwrap_or(raw_value)
                .to_owned();
            settings.push((key, value));
        }
    }
    settings
}

/// T-NPMRC-01: Detect duplicate keys — pnpm uses last-wins, which may mask earlier values.
fn check_duplicate_keys(
    settings: &NpmrcSettings,
    npmrc_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for (key, _) in settings {
        let entry = counts.entry(key.as_str()).or_insert(0);
        *entry = entry.saturating_add(1);
    }
    for (key, count) in &counts {
        if *count > 1 {
            results.push(CheckResult::from_parts(
                "T-NPMRC-01".to_owned(),
                Severity::Error,
                format!("Duplicate key `{key}` in `.npmrc`"),
                format!(
                    "Duplicate key `{key}` in .npmrc \u{2014} pnpm uses the last value, \
                     which may differ from earlier entries."
                ),
                Some(npmrc_path.display().to_string()),
                None,
                false,
            ));
        }
    }
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
        // Use rfind (last match) because pnpm uses last-wins for duplicate keys
        let found = settings.iter().rev().find(|(k, _)| k == key);
        match found {
            Some((_, val)) if val == expected_val => {
                // Correct — no output needed
            }
            Some((_, val)) => {
                // T13: Weaker value
                results.push(CheckResult::from_parts(
                    "T13".to_owned(),
                    Severity::Error,
                    format!("`.npmrc` setting `{key}` has wrong value"),
                    format!(
                        "`{key}` is set to `{val}` but should be `{expected_val}`. \
                         This setting controls pnpm strictness — a weaker value reduces protection against \
                         dependency conflicts or supply chain issues. Update `.npmrc` to set `{key}={expected_val}`."
                    ),
                    Some(npmrc_path.display().to_string()),
                    None,
                    false,
                ));
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
            results.push(
                CheckResult::from_parts(
                    "T14".to_owned(),
                    Severity::Info,
                    format!("Extra `.npmrc` setting: `{key}`"),
                    format!(
                    "Non-baseline `.npmrc` setting `{key}={val}`. This setting is not in the guardrail baseline. \
                     Verify it is intentional and document why it's needed."
                ),
                    Some(npmrc_path.display().to_string()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
    }
}
