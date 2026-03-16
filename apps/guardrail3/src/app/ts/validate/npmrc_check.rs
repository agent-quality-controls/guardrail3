use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

type NpmrcExpectation<'a> = (&'a str, &'a str);

#[allow(clippy::too_many_lines)] // reason: npmrc settings validation
#[allow(clippy::string_slice)] // reason: parsing known ASCII key=value pairs
pub fn check_npmrc(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let npmrc_path = path.join(".npmrc");
    if !npmrc_path.exists() {
        results.push(CheckResult {
            id: "T11".to_owned(),
            severity: Severity::Error,
            title: ".npmrc missing".to_owned(),
            message: "No .npmrc found at project root".to_owned(),
            file: Some(path.display().to_string()),
            line: None,
        });
        return;
    }

    results.push(CheckResult {
        id: "T11".to_owned(),
        severity: Severity::Info,
        title: ".npmrc exists".to_owned(),
        message: "Found at project root".to_owned(),
        file: Some(npmrc_path.display().to_string()),
        line: None,
    });

    let Some(content) = fs.read_file(&npmrc_path) else {
        return;
    };

    #[allow(clippy::type_complexity)] // reason: legitimate complex type
    // Parse key=value pairs
    let mut settings: Vec<(String, String)> = Vec::new();
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

    let expected_keys: Vec<&str> = expected.iter().map(|(k, _)| *k).collect();

    // T12: Check each expected setting
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
                    title: format!(".npmrc {key} wrong value"),
                    message: format!("Expected \"{expected_val}\", got \"{val}\""),
                    file: Some(npmrc_path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "T12".to_owned(),
                    severity: Severity::Error,
                    title: format!(".npmrc {key} missing"),
                    message: format!("Expected {key}={expected_val}"),
                    file: Some(npmrc_path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    // T14: Extra settings not in expected list
    for (key, val) in &settings {
        if !expected_keys.contains(&key.as_str()) {
            results.push(CheckResult {
                id: "T14".to_owned(),
                severity: Severity::Info,
                title: format!(".npmrc extra setting: {key}"),
                message: format!("{key}={val}"),
                file: Some(npmrc_path.display().to_string()),
                line: None,
            });
        }
    }
}
