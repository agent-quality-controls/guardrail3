use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};

pub fn check_skip_entries(table: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    let Some(bans) = table.get("bans") else {
        return;
    };

    if let Some(skip) = bans.get("skip").and_then(|s| s.as_array()) {
        for entry in skip {
            // Try 0.19+ format: { crate = "name@version" }
            let (name, version) =
                if let Some(crate_field) = entry.get("crate").and_then(|c| c.as_str()) {
                    let (n, v) = crate_field
                        .split_once('@')
                        .map_or((crate_field, "*"), |parts| parts);
                    (n.to_owned(), v.to_owned())
                } else if let Some(s) = entry.as_str() {
                    // Plain string entry
                    (s.to_owned(), "*".to_owned())
                } else {
                    // Fall back to older format: { name = "...", version = "..." }
                    let n = entry
                        .get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("unknown");
                    let v = entry.get("version").and_then(|v| v.as_str()).unwrap_or("*");
                    (n.to_owned(), v.to_owned())
                };

            let reason = entry.get("reason").and_then(|r| r.as_str()).unwrap_or("");

            if reason.is_empty() {
                results.push(CheckResult::from_parts(
    "R19".to_owned(),
    Severity::Warn,
    "Skip entry without reason".to_owned(),
    format!("Dependency `{name}` ({version}) is in deny.toml [bans.skip] without a `reason` field. Add `reason = \"...\"` so reviewers understand why the duplicate is accepted."),
    Some(file_path.display().to_string()),
    None,
    false,
                ));
            }

            results.push(CheckResult {
                id: "R19".to_owned(),
                severity: Severity::Info,
                title: "deny.toml skip entry".to_owned(),
                message: format!("Dependency `{name}` ({version}) is banned but skipped in deny.toml [bans.skip]{reason_suffix}. This is an approved exception to the duplicate-version ban — the crate is a transitive dependency that cannot be deduplicated. No action needed.", reason_suffix = if reason.is_empty() { String::new() } else { format!(" because {reason}") }),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        }
    },
)

pub fn check_advisory_ignores(
    table: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let Some(advisories) = table.get("advisories") else {
        return;
    };

    if let Some(ignore) = advisories.get("ignore").and_then(|i| i.as_array()) {
        for entry in ignore {
            // Advisory ignore entries can be plain strings or tables with id + reason
            let (id, reason) = if let Some(s) = entry.as_str() {
                (s.to_owned(), None)
            } else {
                let id = entry
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let reason = entry.get("reason").and_then(|v| v.as_str());
                (id.to_owned(), reason)
            };

            if reason.is_none() {
                results.push(CheckResult::from_parts(
    "R20".to_owned(),
    Severity::Warn,
    "Advisory ignore without reason".to_owned(),
    format!("Security advisory `{id}` is ignored in deny.toml [advisories.ignore] without a `reason` field. Add `reason = \"...\"` so reviewers understand why the advisory is suppressed."),
    Some(file_path.display().to_string()),
    None,
    false,
                ));
            }

            results.push(CheckResult {
                id: "R20".to_owned(),
                severity: Severity::Info,
                title: "Advisory ignore".to_owned(),
                message: format!("Security advisory `{id}` is explicitly ignored in deny.toml [advisories.ignore]. This suppresses a known vulnerability alert — verify the advisory does not apply to your usage or has an accepted risk."),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    },
)
