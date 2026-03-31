use std::collections::BTreeSet;
use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::{FileSystem, ToolChecker};

/// Run all repo-level release checks (R-REL-01 through R-REL-08).
pub fn check_repo_level(
    fs: &dyn FileSystem,
    tc: &dyn ToolChecker,
    workspace_root: &Path,
    publishable_names: &BTreeSet<String>,
    results: &mut Vec<CheckResult>,
) {
    check_license_file(workspace_root, results);
    check_release_plz_toml(fs, workspace_root, publishable_names, results);
    check_cliff_toml(workspace_root, results);
    check_workflow_contains(
        fs,
        workspace_root,
        "release-plz",
        "R-REL-05",
        "Release workflow found",
        "A workflow references release-plz",
        "No release workflow",
        "No .github/workflows/*.yml containing \"release-plz\"",
        results,
    );
    check_workflow_contains(
        fs,
        workspace_root,
        "cargo publish --dry-run",
        "R-REL-06",
        "Publish dry-run in CI",
        "A workflow contains \"cargo publish --dry-run\"",
        "No publish dry-run in CI",
        "No workflow with \"cargo publish --dry-run\"",
        results,
    );
    check_workflow_contains(
        fs,
        workspace_root,
        "CARGO_REGISTRY_TOKEN",
        "R-REL-07",
        "CARGO_REGISTRY_TOKEN referenced",
        "A workflow references CARGO_REGISTRY_TOKEN",
        "No CARGO_REGISTRY_TOKEN in workflows",
        "No workflow references CARGO_REGISTRY_TOKEN",
        results,
    );
    check_semver_checks_installed(tc, results);
}

// --- R-REL-01: LICENSE file at repo root ---

pub fn check_license_file(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let license_names = ["LICENSE", "LICENSE-MIT", "LICENSE-APACHE", "LICENSE.md"];

    let found = license_names
        .iter()
        .any(|name| workspace_root.join(name).exists());

    if found {
        results.push(CheckResult::from_parts(
    "R-REL-01".to_owned(),
    Severity::Info,
    "LICENSE file exists".to_owned(),
    "LICENSE file found at repo root. Required for open-source publishing and legal compliance. No action needed.".to_owned(),
    Some(workspace_root.display().to_string()),
    None,
    false,
        }.as_inventory());
    } else {
        results.push(CheckResult::from_parts(
    "R-REL-01".to_owned(),
    Severity::Error,
    "LICENSE file missing".to_owned(),
    "No LICENSE file at repo root. Without a license, the code is legally unusable by others. Create `LICENSE` (or `LICENSE-MIT`, `LICENSE-APACHE`) with your license text."
                .to_owned(),
    Some(workspace_root.display().to_string()),
    None,
    false,
        ));
    }

// --- R-REL-02 + R-REL-03: release-plz.toml ---

pub fn check_release_plz_toml(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    publishable_names: &BTreeSet<String>,
    results: &mut Vec<CheckResult>,
) {
    let plz_path = workspace_root.join("release-plz.toml");
    if !plz_path.exists() {
        results.push(CheckResult::from_parts(
    "R-REL-02".to_owned(),
    Severity::Warn,
    "release-plz.toml missing".to_owned(),
    "No release-plz.toml at repo root".to_owned(),
    Some(workspace_root.display().to_string()),
    None,
    false,
        ));
        return;
    }

    results.push(
        CheckResult::from_parts(
            "R-REL-02".to_owned(),
            Severity::Info,
            "release-plz.toml exists".to_owned(),
            "Found at repo root".to_owned(),
            Some(plz_path.display().to_string()),
            None,
            false,
        )
        .as_inventory(),
    );

    let Some(table) = parse_release_plz_toml(fs, &plz_path, results) else {
        return;
    };

    validate_release_plz_content(&table, &plz_path, publishable_names, results);,
)

fn parse_release_plz_toml(
    fs: &dyn FileSystem,
    plz_path: &Path,
    results: &mut Vec<CheckResult>,
) -> Option<toml::Value> {
    let Some(content) = fs.read_file(plz_path) else {
        results.push(CheckResult::from_parts(
    "R-REL-03".to_owned(),
    Severity::Warn,
    "release-plz.toml unreadable".to_owned(),
    "Could not read file".to_owned(),
    Some(plz_path.display().to_string()),
    None,
    false,
        ));
        return None;
    };

    match content.parse() {
        Ok(v) => Some(v),
        Err(e) => {
            results.push(CheckResult::from_parts(
    "R-REL-03".to_owned(),
    Severity::Warn,
    "release-plz.toml invalid TOML".to_owned(),
    format!("Parse error: {e}"),
    Some(plz_path.display().to_string()),
    None,
    false,
            });
            None
        }
    },
)

fn validate_release_plz_content(
    table: &toml::Value,
    plz_path: &Path,
    publishable_names: &BTreeSet<String>,
    results: &mut Vec<CheckResult>,
) {
    // Check [workspace] section
    if table.get("workspace").is_none() {
        results.push(CheckResult::from_parts(
    "R-REL-03".to_owned(),
    Severity::Warn,
    "release-plz.toml missing [workspace]".to_owned(),
    "No [workspace] section found".to_owned(),
    Some(plz_path.display().to_string()),
    None,
    false,
        ));
        return;
    }

    // Check [[package]] entries cover all publishable crates
    let configured_names: BTreeSet<String> = table
        .get("package")
        .and_then(|p| p.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|entry| {
                    entry
                        .get("name")
                        .and_then(|n| n.as_str())
                        .map(std::borrow::ToOwned::to_owned)
                })
                .collect()
        })
        .unwrap_or_default();

    let missing: BTreeSet<_> = publishable_names.difference(&configured_names).collect();

    if missing.is_empty() {
        results.push(
            CheckResult::from_parts(
                "R-REL-03".to_owned(),
                Severity::Info,
                "release-plz.toml covers all crates".to_owned(),
                "All publishable crates have [[package]] entries".to_owned(),
                Some(plz_path.display().to_string()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        for name in &missing {
            results.push(CheckResult::from_parts(
    "R-REL-03".to_owned(),
    Severity::Warn,
    format!("release-plz.toml missing package \"{name}\""),
    format!("Publishable crate \"{name}\" has no [[package]] entry"),
    Some(plz_path.display().to_string()),
    None,
    false,
            });
        }
    },
)

// --- R-REL-04: cliff.toml ---

pub fn check_cliff_toml(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    super::code_quality_checks::check_file_exists_at_root(
        workspace_root,
        "cliff.toml",
        "R-REL-04",
        "cliff.toml exists",
        "cliff.toml missing",
        results,
    );,
)

// --- R-REL-05 through R-REL-07: workflow checks ---

/// (filename, content) pairs from workflow YAML files.
pub type WorkflowFiles = Vec<(String, String)>;

/// Read all YAML files from .github/workflows/ and return their contents.
pub fn read_workflow_files(fs: &dyn FileSystem, workspace_root: &Path) -> WorkflowFiles {
    let workflows_dir = workspace_root.join(".github").join("workflows");
    if !workflows_dir.exists() {
        return Vec::new();
    }

    let mut files = Vec::new();
    for entry in fs.list_dir(&workflows_dir) {
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_owned();
        let is_yaml = Path::new(&name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("yml") || ext.eq_ignore_ascii_case("yaml"));
        if is_yaml {
            if let Some(content) = fs.read_file(&path) {
                files.push((name, content));
            }
        }
    }
    files,
)

/// Check if any workflow file contains a pattern, emitting an appropriate result.
pub fn check_workflow_contains(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    pattern: &str,
    check_id: &str,
    found_title: &str,
    found_msg: &str,
    missing_title: &str,
    missing_msg: &str,
    results: &mut Vec<CheckResult>,
) {
    WorkflowPresenceCheck {
        pattern,
        check_id,
        found_title,
        found_msg,
        missing_title,
        missing_msg,
    }
    .run(&read_workflow_files(fs, workspace_root), results);,
)

struct WorkflowPresenceCheck<'a> {
    pattern: &'a str,
    check_id: &'a str,
    found_title: &'a str,
    found_msg: &'a str,
    missing_title: &'a str,
    missing_msg: &'a str,
}

impl<'a> WorkflowPresenceCheck<'a> {
    fn run(self, workflows: &WorkflowFiles, results: &mut Vec<CheckResult>) {
        if workflows
            .iter()
            .any(|(_, content)| content.contains(self.pattern))
        {
            results.push(
                CheckResult::from_parts(
                    self.check_id.to_owned(),
                    Severity::Info,
                    self.found_title.to_owned(),
                    format!(
                        "{}. CI workflow configuration confirmed. No action needed.",
                        self.found_msg
                    ),
                    None,
                    None,
                    false,
                )
                .as_inventory(),
            );
        } else {
            results.push(CheckResult::from_parts(
    self.check_id.to_owned(),
    Severity::Warn,
    self.missing_title.to_owned(),
    format!(
                    "{}. This CI check is recommended for automated release quality enforcement. Add it to a `.github/workflows/*.yml` file.",
                    self.missing_msg
                ),
    None,
    None,
    false,
            ));
        }
    },
)

// --- R-REL-08: cargo-semver-checks installed ---

/// Check if a CLI tool is installed on PATH.
pub fn check_tool_installed(
    tc: &dyn ToolChecker,
    tool_name: &str,
    check_id: &str,
    install_cmd: &str,
    results: &mut Vec<CheckResult>,
) {
    if tc.is_installed(tool_name) {
        results.push(CheckResult::from_parts(
    check_id.to_owned(),
    Severity::Info,
    format!("{tool_name} installed"),
    format!("`{tool_name}` found on PATH. Required tool for release/quality checks is available. No action needed."),
    None,
    None,
    false,
        }.as_inventory());
    } else {
        results.push(CheckResult::from_parts(
    check_id.to_owned(),
    Severity::Warn,
    format!("{tool_name} not installed"),
    format!("`{tool_name}` not found on PATH. This tool is needed for release quality checks. Install with: `{install_cmd}`"),
    None,
    None,
    false,
        ));
    }

pub fn check_semver_checks_installed(tc: &dyn ToolChecker, results: &mut Vec<CheckResult>) {
    check_tool_installed(
        tc,
        "cargo-semver-checks",
        "R-REL-08",
        "cargo install cargo-semver-checks",
        results,
    );,
)
