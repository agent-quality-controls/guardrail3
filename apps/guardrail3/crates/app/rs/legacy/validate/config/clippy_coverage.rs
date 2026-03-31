use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

pub use guardrail3_app_rs_family_clippy::clippy_support::{
    EXPECTED_METHOD_BANS, EXPECTED_TYPE_BANS,
};

pub fn check(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    _profile: Option<&str>,
    clippy_tomls: &[PathBuf],
) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // Find the root clippy.toml from crawler data
    let root_clippy = clippy_tomls
        .iter()
        .find(|p| p.parent() == Some(workspace_root));

    let Some(clippy_path) = root_clippy else {
        results.push(CheckResult::from_parts(
    "R4".to_owned(),
    Severity::Error,
    "Cannot check clippy bans".to_owned(),
    "clippy.toml not found".to_owned(),
    Some(workspace_root.display().to_string()),
    None,
    false,
        ));
        return results;
    };

    let content = match fs.read_file_err(clippy_path) {
        Ok(content) => content,
        Err(e) => {
            results.push(CheckResult::from_parts(
    "R4".to_owned(),
    Severity::Error,
    "clippy.toml unreadable".to_owned(),
    format!("Failed to read: {e}"),
    Some(clippy_path.display().to_string()),
    None,
    false,
            });
            return results;
        }
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult::from_parts(
    "R4".to_owned(),
    Severity::Error,
    "clippy.toml parse error".to_owned(),
    format!("Invalid TOML: {e}"),
    Some(clippy_path.display().to_string()),
    None,
    false,
            });
            return results;
        }
    };

    // All profiles (service, library) use the same expected bans.
    // Unknown/missing profiles default to service (the most comprehensive set).
    let expected_methods = EXPECTED_METHOD_BANS;
    let expected_types = EXPECTED_TYPE_BANS;

    // Check disallowed-methods: R4=missing, R6=extras
    check_ban_list(
        &table,
        "disallowed-methods",
        expected_methods,
        "R4",
        "R6",
        "method ban",
        clippy_path,
        &mut results,
    );

    // Check disallowed-types: R5=missing, R7=extras
    check_ban_list(
        &table,
        "disallowed-types",
        expected_types,
        "R5",
        "R7",
        "type ban",
        clippy_path,
        &mut results,
    );

    results,
)

fn check_ban_list(
    table: &toml::Value,
    key: &str,
    expected: &[&str],
    missing_id: &str,
    extra_id: &str,
    label: &str,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let check = BanListCheck {
        table,
        key,
        expected,
        missing_id,
        extra_id,
        label,
        file_path,
    };
    check.run(results);,
)

struct BanListCheck<'a> {
    table: &'a toml::Value,
    key: &'a str,
    expected: &'a [&'a str],
    missing_id: &'a str,
    extra_id: &'a str,
    label: &'a str,
    file_path: &'a Path,
)

impl<'a> BanListCheck<'a> {
    fn run(&self, results: &mut Vec<CheckResult>) {
        let Some(bans) = self.table.get(self.key).and_then(|v| v.as_array()) else {
            results.push(CheckResult::from_parts(
    self.missing_id.to_owned(),
    Severity::Error,
    format!("No {} section", self.key),
    format!("{} array missing from clippy.toml", self.key),
    Some(self.file_path.display().to_string()),
    None,
    false,
            ));
            return;
        };

        let found_paths = self.collect_found_paths(bans, results);
        let expected_set: BTreeSet<String> =
            self.expected.iter().map(|s| (*s).to_owned()).collect();

        self.report_expected_coverage(&expected_set, &found_paths, results);
        self.report_extra_bans(&expected_set, &found_paths, results);
    }

    fn collect_found_paths(
        &self,
        bans: &[toml::Value],
        results: &mut Vec<CheckResult>,
    ) -> BTreeSet<String> {
        let mut found_paths = BTreeSet::new();
        for ban in bans {
            if let Some(path) = ban.get("path").and_then(|p| p.as_str()) {
                if ban.get("reason").and_then(|r| r.as_str()).is_none() {
                    self.push_missing_reason_table_result(path, results);
                }
                let _ = found_paths.insert(path.to_owned());
            } else if let Some(path) = ban.as_str() {
                self.push_missing_reason_string_result(path, results);
                let _ = found_paths.insert(path.to_owned());
            }
        }
        found_paths
    }

    fn push_missing_reason_table_result(&self, path: &str, results: &mut Vec<CheckResult>) {
        results.push(CheckResult {
            id: self.missing_id.to_owned(),
            severity: Severity::Warn,
            title: format!("{} without reason", self.label),
            message: format!(
                "`{path}` is banned in clippy.toml {} but has no `reason` field. Add `reason = \"...\"` so developers understand why this API is banned and what alternative to use.",
                self.key
            ),
            file: Some(self.file_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    fn push_missing_reason_string_result(&self, path: &str, results: &mut Vec<CheckResult>) {
        results.push(CheckResult {
            id: self.missing_id.to_owned(),
            severity: Severity::Warn,
            title: format!("{} without reason", self.label),
            message: format!(
                "`{path}` is banned in clippy.toml {} as a plain string with no `reason` field. Use table format `{{ path = \"{path}\", reason = \"...\" }}` so developers understand why this API is banned.",
                self.key
            ),
            file: Some(self.file_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    fn report_expected_coverage(
        &self,
        expected_set: &BTreeSet<String>,
        found_paths: &BTreeSet<String>,
        results: &mut Vec<CheckResult>,
    ) {
        for expected in expected_set {
            if found_paths.contains(expected) {
                results.push(CheckResult {
                    id: self.missing_id.to_owned(),
                    severity: Severity::Info,
                    title: format!("{} present", self.label),
                    message: format!(
                        "`{expected}` is banned in clippy.toml {}. Calls to this API are flagged at compile time. No action needed.",
                        self.key
                    ),
                    file: Some(self.file_path.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory());
            } else {
                results.push(CheckResult {
                    id: self.missing_id.to_owned(),
                    severity: Severity::Error,
                    title: format!("Missing {}", self.label),
                    message: format!(
                        "`{expected}` is not banned in clippy.toml {}. Without this ban, code can use this API unchecked, bypassing guardrail enforcement. Add it to the `{}` array in clippy.toml or run `guardrail3 generate` to regenerate.",
                        self.key, self.key
                    ),
                    file: Some(self.file_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }

    fn report_extra_bans(
        &self,
        expected_set: &BTreeSet<String>,
        found_paths: &BTreeSet<String>,
        results: &mut Vec<CheckResult>,
    ) {
        for found in found_paths {
            if !expected_set.contains(found) {
                results.push(CheckResult {
                    id: self.extra_id.to_owned(),
                    severity: Severity::Info,
                    title: format!("Extra {}", self.label),
                    message: format!(
                        "Additional ban `{found}` in clippy.toml {} beyond the guardrail3 baseline. Project-specific ban — no action needed.",
                        self.key
                    ),
                    file: Some(self.file_path.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory());
            }
        }
    },
)
