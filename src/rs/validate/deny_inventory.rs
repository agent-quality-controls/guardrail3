use std::path::Path;

use crate::report::types::{CheckResult, Severity};

pub fn check_skip_entries(table: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    let Some(bans) = table.get("bans") else {
        return;
    };

    if let Some(skip) = bans.get("skip").and_then(|s| s.as_array()) {
        for entry in skip {
            // Try 0.19+ format: { crate = "name@version" }
            let (name, version) = if let Some(crate_field) =
                entry.get("crate").and_then(|c| c.as_str())
            {
                let parts: Vec<&str> = crate_field.splitn(2, '@').collect();
                #[allow(clippy::indexing_slicing)] // reason: splitn(2) guarantees index 0 exists
                let n = parts[0];
                let v = parts.get(1).copied().unwrap_or("*");
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
            let message = if reason.is_empty() {
                format!("{name} {version}")
            } else {
                format!("{name} {version} — {reason}")
            };

            results.push(CheckResult {
                id: "R19".to_owned(),
                severity: Severity::Info,
                title: "Skip entry".to_owned(),
                message,
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }
}

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
            let id = entry.as_str().unwrap_or("unknown");
            results.push(CheckResult {
                id: "R20".to_owned(),
                severity: Severity::Info,
                title: "Advisory ignore".to_owned(),
                message: id.to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Bug 6: deny.toml skip entry parsing ----

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    #[allow(clippy::expect_used)] // reason: test setup parses TOML with expect
    fn skip_entry_parses_crate_at_version_format() {
        let deny_content = r#"
[bans]
skip = [
    { crate = "windows-sys@0.60.2", reason = "transitive dep conflict" },
]
"#;
        let table: toml::Value = deny_content.parse().expect("valid TOML");
        let mut results = Vec::new();
        let path = Path::new("deny.toml");
        check_skip_entries(&table, path, &mut results);
        assert!(
            !results.is_empty(),
            "Should produce a result for skip entry"
        );
        assert!(
            results[0].message.contains("windows-sys"),
            "Should parse crate name from crate@version format, got: {}",
            results[0].message
        );
        assert!(
            results[0].message.contains("0.60.2"),
            "Should parse version from crate@version format, got: {}",
            results[0].message
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    #[allow(clippy::expect_used)] // reason: test setup parses TOML with expect
    fn skip_entry_parses_old_name_version_format() {
        let deny_content = r#"
[bans]
skip = [
    { name = "windows-sys", version = "0.60.2" },
]
"#;
        let table: toml::Value = deny_content.parse().expect("valid TOML");
        let mut results = Vec::new();
        let path = Path::new("deny.toml");
        check_skip_entries(&table, path, &mut results);
        assert!(!results.is_empty());
        assert!(
            results[0].message.contains("windows-sys"),
            "Should parse name from old format"
        );
        assert!(
            results[0].message.contains("0.60.2"),
            "Should parse version from old format"
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    #[allow(clippy::expect_used)] // reason: test setup parses TOML with expect
    fn skip_entry_parses_plain_string() {
        let deny_content = r#"
[bans]
skip = ["some-crate"]
"#;
        let table: toml::Value = deny_content.parse().expect("valid TOML");
        let mut results = Vec::new();
        let path = Path::new("deny.toml");
        check_skip_entries(&table, path, &mut results);
        assert!(!results.is_empty());
        assert!(results[0].message.contains("some-crate"));
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
    #[allow(clippy::expect_used)] // reason: test setup parses TOML with expect
    fn skip_entry_includes_reason() {
        let deny_content = r#"
[bans]
skip = [
    { crate = "foo@1.0.0", reason = "needed for compat" },
]
"#;
        let table: toml::Value = deny_content.parse().expect("valid TOML");
        let mut results = Vec::new();
        let path = Path::new("deny.toml");
        check_skip_entries(&table, path, &mut results);
        assert!(!results.is_empty());
        assert!(
            results[0].message.contains("needed for compat"),
            "Should include reason in message"
        );
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test setup parses TOML with expect
    fn no_skip_section_produces_no_results() {
        let deny_content = r"
[bans]
deny = []
";
        let table: toml::Value = deny_content.parse().expect("valid TOML");
        let mut results = Vec::new();
        let path = Path::new("deny.toml");
        check_skip_entries(&table, path, &mut results);
        assert!(results.is_empty());
    }
}
