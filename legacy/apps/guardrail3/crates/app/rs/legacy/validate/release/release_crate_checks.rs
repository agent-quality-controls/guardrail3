use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};

use super::release_checks::CrateInfo;
use super::release_crate_deps;
use guardrail3_outbound_traits::{FileSystem, ToolChecker};

/// Run all per-crate checks on a single publishable crate.
pub fn check_per_crate(
    fs: &dyn FileSystem,
    tc: &dyn ToolChecker,
    krate: &CrateInfo,
    publishable_names: &BTreeSet<String>,
    version_map: &BTreeMap<String, String>,
    thorough: bool,
    results: &mut Vec<CheckResult>,
) {
    let pkg = krate.table.get("package");
    let file_str = krate.cargo_toml_path.display().to_string();
    let file = Some(file_str.as_str());

    check_required_string_field(pkg, "description", "R-PUB-01", &krate.name, file, results);
    check_license(pkg, &krate.name, file, results);
    check_required_string_field(pkg, "repository", "R-PUB-03", &krate.name, file, results);
    check_readme(fs, pkg, krate, file, results);
    release_crate_deps::check_keywords(pkg, &krate.name, file, results);
    release_crate_deps::check_categories(pkg, &krate.name, file, results);
    check_version(pkg, &krate.name, file, results);
    release_crate_deps::check_path_deps(&krate.table, krate, publishable_names, file, results);
    release_crate_deps::check_version_consistency(&krate.table, krate, version_map, file, results);

    if thorough {
        release_crate_deps::check_publish_dry_run(tc, krate, results);
    }
}

/// Check that a required string field exists and is non-empty in [package].
pub fn check_required_string_field(
    pkg: Option<&toml::Value>,
    field_name: &str,
    check_id: &str,
    crate_name: &str,
    file: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let has_field = pkg
        .and_then(|p| p.get(field_name))
        .and_then(|v| v.as_str())
        .is_some_and(|s| !s.is_empty());

    results.push(CheckResult::from_parts(
    check_id.to_owned(),
    if has_field {
            Severity::Info
        } else {
            Severity::Error
        },
    if has_field {
            format!("{crate_name}: {field_name} present")
        } else {
            format!("{crate_name}: missing {field_name}")
        },
    if has_field {
            format!("Cargo.toml [package].{field_name} is set. Required for crates.io publishing. No action needed.")
        } else {
            format!("[package].{field_name} is missing or empty in Cargo.toml. Required for crates.io publishing — `cargo publish` will fail without it. Add `{field_name} = \"...\"` to [package].")
        },
    file.map(std::borrow::ToOwned::to_owned),
    None,
    false,
    }.as_inventory());,
)

// --- R-PUB-02: license ---

pub fn check_license(
    pkg: Option<&toml::Value>,
    name: &str,
    file: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let has_license = pkg
        .and_then(|p| p.get("license"))
        .and_then(|l| l.as_str())
        .is_some_and(|s| !s.is_empty());
    let has_license_file = pkg
        .and_then(|p| p.get("license-file"))
        .and_then(|l| l.as_str())
        .is_some_and(|s| !s.is_empty());
    let ok = has_license || has_license_file;

    results.push(CheckResult::from_parts(
    "R-PUB-02".to_owned(),
    if ok { Severity::Info } else { Severity::Error },
    if ok {
            format!("{name}: license present")
        } else {
            format!("{name}: missing license")
        },
    if ok {
            "Cargo.toml has `license` or `license-file` field. Required for crates.io publishing and legal compliance. No action needed.".to_owned()
        } else {
            "No license specified in Cargo.toml. crates.io requires a license for publishing, and users cannot legally use unlicensed code. Add `license = \"MIT\"` (or your license) to [package].".to_owned()
        },
    file.map(std::borrow::ToOwned::to_owned),
    None,
    false,
    }.as_inventory());,
)

// --- R-PUB-04 + R-PUB-05: readme ---

pub fn check_readme(
    fs: &dyn FileSystem,
    pkg: Option<&toml::Value>,
    krate: &CrateInfo,
    file: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let readme_field = pkg.and_then(|p| p.get("readme")).and_then(|r| r.as_str());

    match readme_field {
        None => {
            let default_readme = krate.dir.join("README.md");
            if default_readme.exists() {
                results.push(
                    CheckResult::from_parts(
                        "R-PUB-04".to_owned(),
                        Severity::Info,
                        format!("{}: readme found (default)", krate.name),
                        "README.md exists at crate root (no explicit readme field)"
                            .to_owned(),
                        file.map(std::borrow::ToOwned::to_owned),
                        None,
                        false,
                    )
                    .as_inventory(),
                );
                check_readme_quality(fs, &default_readme, &krate.name, file, results);
            } else {
                results.push(CheckResult::from_parts(
    "R-PUB-04".to_owned(),
    Severity::Warn,
    format!("{}: readme missing", krate.name),
    "No readme field and no README.md at crate root".to_owned(),
    file.map(std::borrow::ToOwned::to_owned),
    None,
    false,
                ));
            }
        }
        Some(readme_path) => {
            let full_path = krate.dir.join(readme_path);
            if full_path.exists() {
                results.push(
                    CheckResult::from_parts(
                        "R-PUB-04".to_owned(),
                        Severity::Info,
                        format!("{}: readme present", krate.name),
                        format!("readme = \"{readme_path}\" exists on disk"),
                        file.map(std::borrow::ToOwned::to_owned),
                        None,
                        false,
                    )
                    .as_inventory(),
                );
                check_readme_quality(fs, &full_path, &krate.name, file, results);
            } else {
                results.push(CheckResult::from_parts(
    "R-PUB-04".to_owned(),
    Severity::Warn,
    format!("{}: readme file not found", krate.name),
    format!("readme = \"{readme_path}\" but file does not exist"),
    file.map(std::borrow::ToOwned::to_owned),
    None,
    false,
                });
            }
        }
    },
)

pub fn check_readme_quality(
    fs: &dyn FileSystem,
    readme_path: &Path,
    name: &str,
    file: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let Some(content) = fs.read_file(readme_path) else {
        return;
    };
    let size = content.len();
    let has_heading = content.lines().any(|line| line.starts_with('#'));

    let (severity, title, message) = if size < 200 {
        (
            Severity::Warn,
            format!("{name}: README is a stub"),
            format!("README is {size} bytes (min 200)"),
        )
    } else if !has_heading {
        (
            Severity::Warn,
            format!("{name}: README has no heading"),
            "README has no line starting with #".to_owned(),
        )
    } else {
        (
            Severity::Info,
            format!("{name}: README looks good"),
            format!("{size} bytes with headings"),
        )
    };

    let result = CheckResult {
        id: "R-PUB-05".to_owned(),
        severity,
        title,
        message,
        file: file.map(std::borrow::ToOwned::to_owned),
        line: None,
        inventory: false,
    };
    results.push(if severity == Severity::Info {
        result.as_inventory()
    } else {
        result
    });,
)

// --- R-PUB-08: semver version ---

pub fn check_version(
    pkg: Option<&toml::Value>,
    name: &str,
    file: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let version_val = pkg.and_then(|p| p.get("version"));

    // Check for version.workspace = true (table form)
    let is_workspace_version = version_val
        .and_then(|v| v.as_table())
        .and_then(|t| t.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);

    let version = if is_workspace_version {
        // version.workspace = true means the version comes from the workspace root.
        // We can't resolve the actual version here (no access to workspace root Cargo.toml),
        // so we treat it as valid — the workspace root's version will be checked separately.
        None
    } else {
        version_val.and_then(|v| v.as_str())
    };

    let (severity, title, message) = if is_workspace_version {
        (
            Severity::Info,
            format!("{name}: version inherited from workspace"),
            "version.workspace = true — version is inherited from workspace root Cargo.toml"
                .to_owned(),
        )
    } else {
        match version {
            None => (
                Severity::Error,
                format!("{name}: version missing"),
                "Cargo.toml [package].version is missing".to_owned(),
            ),
            Some(v) if release_crate_deps::is_valid_semver(v) => (
                Severity::Info,
                format!("{name}: valid semver"),
                format!("version = \"{v}\""),
            ),
            Some(v) => (
                Severity::Error,
                format!("{name}: invalid semver"),
                format!("version = \"{v}\" does not parse as X.Y.Z"),
            ),
        }
    };

    let result = CheckResult {
        id: "R-PUB-08".to_owned(),
        severity,
        title,
        message,
        file: file.map(std::borrow::ToOwned::to_owned),
        line: None,
        inventory: false,
    };
    results.push(if severity == Severity::Info {
        result.as_inventory()
    } else {
        result
    });
}
