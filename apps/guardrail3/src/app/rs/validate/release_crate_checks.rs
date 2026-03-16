use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

use super::release_checks::CrateInfo;
use super::release_crate_deps;
use crate::ports::outbound::{FileSystem, ToolChecker};

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

    results.push(CheckResult {
        id: check_id.to_owned(),
        severity: if has_field {
            Severity::Info
        } else {
            Severity::Error
        },
        title: if has_field {
            format!("{crate_name}: {field_name} present")
        } else {
            format!("{crate_name}: missing {field_name}")
        },
        message: if has_field {
            format!("Cargo.toml has [package].{field_name}")
        } else {
            format!("Cargo.toml [package].{field_name} is missing or empty")
        },
        file: file.map(std::borrow::ToOwned::to_owned),
        line: None,
    });
}

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

    results.push(CheckResult {
        id: "R-PUB-02".to_owned(),
        severity: if ok { Severity::Info } else { Severity::Error },
        title: if ok {
            format!("{name}: license present")
        } else {
            format!("{name}: missing license")
        },
        message: if ok {
            "Cargo.toml has license or license-file".to_owned()
        } else {
            "Cargo.toml [package] has neither license nor license-file".to_owned()
        },
        file: file.map(std::borrow::ToOwned::to_owned),
        line: None,
    });
}

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
                results.push(CheckResult {
                    id: "R-PUB-04".to_owned(),
                    severity: Severity::Info,
                    title: format!("{}: readme found (default)", krate.name),
                    message: "README.md exists at crate root (no explicit readme field)".to_owned(),
                    file: file.map(std::borrow::ToOwned::to_owned),
                    line: None,
                });
                check_readme_quality(fs, &default_readme, &krate.name, file, results);
            } else {
                results.push(CheckResult {
                    id: "R-PUB-04".to_owned(),
                    severity: Severity::Warn,
                    title: format!("{}: readme missing", krate.name),
                    message: "No readme field and no README.md at crate root".to_owned(),
                    file: file.map(std::borrow::ToOwned::to_owned),
                    line: None,
                });
            }
        }
        Some(readme_path) => {
            let full_path = krate.dir.join(readme_path);
            if full_path.exists() {
                results.push(CheckResult {
                    id: "R-PUB-04".to_owned(),
                    severity: Severity::Info,
                    title: format!("{}: readme present", krate.name),
                    message: format!("readme = \"{readme_path}\" exists on disk"),
                    file: file.map(std::borrow::ToOwned::to_owned),
                    line: None,
                });
                check_readme_quality(fs, &full_path, &krate.name, file, results);
            } else {
                results.push(CheckResult {
                    id: "R-PUB-04".to_owned(),
                    severity: Severity::Warn,
                    title: format!("{}: readme file not found", krate.name),
                    message: format!("readme = \"{readme_path}\" but file does not exist"),
                    file: file.map(std::borrow::ToOwned::to_owned),
                    line: None,
                });
            }
        }
    }
}

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

    results.push(CheckResult {
        id: "R-PUB-05".to_owned(),
        severity,
        title,
        message,
        file: file.map(std::borrow::ToOwned::to_owned),
        line: None,
    });
}

// --- R-PUB-08: semver version ---

pub fn check_version(
    pkg: Option<&toml::Value>,
    name: &str,
    file: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let version = pkg.and_then(|p| p.get("version")).and_then(|v| v.as_str());

    let (severity, title, message) = match version {
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
    };

    results.push(CheckResult {
        id: "R-PUB-08".to_owned(),
        severity,
        title,
        message,
        file: file.map(std::borrow::ToOwned::to_owned),
        line: None,
    });
}

