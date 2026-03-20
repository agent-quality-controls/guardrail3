use std::collections::{BTreeMap, BTreeSet};

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::ToolChecker;

use super::release_checks::CrateInfo;

// --- R-PUB-09: cargo publish --dry-run ---

pub fn check_publish_dry_run(
    tc: &dyn ToolChecker,
    krate: &CrateInfo,
    results: &mut Vec<CheckResult>,
) {
    match tc.run_cargo_publish_dry_run(&krate.dir) {
        Some(stderr) if stderr.is_empty() || !stderr.contains("error") => {
            results.push(
                CheckResult {
                    id: "R-PUB-09".to_owned(),
                    severity: Severity::Info,
                    title: format!("{}: publish dry-run passed", krate.name),
                    message: "cargo publish --dry-run succeeded".to_owned(),
                    file: Some(krate.cargo_toml_path.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        }
        Some(stderr) => {
            results.push(CheckResult {
                id: "R-PUB-09".to_owned(),
                severity: Severity::Error,
                title: format!("{}: publish dry-run failed", krate.name),
                message: format!(
                    "cargo publish --dry-run failed: {}",
                    stderr.lines().take(3).collect::<Vec<_>>().join("; ")
                ),
                file: Some(krate.cargo_toml_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R-PUB-09".to_owned(),
                severity: Severity::Error,
                title: format!("{}: publish dry-run error", krate.name),
                message: "Could not run cargo publish --dry-run".to_owned(),
                file: Some(krate.cargo_toml_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

// --- R-PUB-10: path deps to non-publishable crates ---

pub fn check_path_deps(
    table: &toml::Value,
    krate: &CrateInfo,
    publishable_names: &BTreeSet<String>,
    file: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let dep_sections = ["dependencies", "build-dependencies"];
    let mut bad_deps = Vec::new();

    for section_name in &dep_sections {
        let Some(section) = table.get(section_name) else {
            continue;
        };
        let Some(deps_table) = section.as_table() else {
            continue;
        };
        for (dep_name, dep_val) in deps_table {
            let has_path = dep_val.get("path").and_then(|p| p.as_str()).is_some();
            if has_path && !publishable_names.contains(dep_name.as_str()) {
                bad_deps.push(format!("{dep_name} (in [{section_name}])"));
            }
        }
    }

    if bad_deps.is_empty() {
        results.push(
            CheckResult {
                id: "R-PUB-10".to_owned(),
                severity: Severity::Info,
                title: format!("{}: path deps OK", krate.name),
                message: "No path dependencies to non-publishable crates".to_owned(),
                file: file.map(std::borrow::ToOwned::to_owned),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        for bad in &bad_deps {
            results.push(CheckResult {
                id: "R-PUB-10".to_owned(),
                severity: Severity::Error,
                title: format!("{}: path dep to non-publishable crate", krate.name),
                message: format!("Depends on {bad} which is not publishable"),
                file: file.map(std::borrow::ToOwned::to_owned),
                line: None,
                inventory: false,
            });
        }
    }
}

// --- R-PUB-11: version consistency ---

/// Check that workspace members depending on each other have compatible versions.
/// If A depends on B with `version` = "X.Y", B's actual version must satisfy that requirement.
pub fn check_version_consistency(
    table: &toml::Value,
    krate: &CrateInfo,
    version_map: &BTreeMap<String, String>,
    file: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let Some(deps) = table.get("dependencies").and_then(|d| d.as_table()) else {
        return;
    };

    for (dep_name, dep_val) in deps {
        if dep_val.get("path").is_none() {
            continue;
        }
        let Some(req) = dep_val.get("version").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(actual) = version_map.get(dep_name.as_str()) else {
            continue;
        };
        if !version_satisfies(actual, req) {
            results.push(CheckResult {
                id: "R-PUB-11".to_owned(),
                severity: Severity::Error,
                title: format!("{}: version mismatch with {dep_name}", krate.name),
                message: format!(
                    "Requires {dep_name} version \"{req}\" but {dep_name} is \"{actual}\""
                ),
                file: file.map(std::borrow::ToOwned::to_owned),
                line: None,
                inventory: false,
            });
        }
    }
}

/// Simple semver compatibility check: does `actual` satisfy requirement `req`?
/// Supports bare version "X.Y.Z", caret "^X.Y.Z" (default), and tilde "~X.Y.Z".
#[allow(clippy::suspicious_operation_groupings)] // reason: tuple comparison (a_minor,a_patch)>=(r_minor,r_patch) is intentional
pub fn version_satisfies(actual: &str, req: &str) -> bool {
    let req_trimmed = req.trim();
    let (prefix, req_ver) = if let Some(stripped) = req_trimmed.strip_prefix('^') {
        ("^", stripped)
    } else if let Some(stripped) = req_trimmed.strip_prefix('~') {
        ("~", stripped)
    } else if let Some(stripped) = req_trimmed.strip_prefix(">=") {
        (">=", stripped)
    } else if let Some(stripped) = req_trimmed.strip_prefix('=') {
        ("=", stripped)
    } else {
        ("^", req_trimmed) // bare version = caret
    };

    let (a_major, a_minor, a_patch) = parse_version_parts(actual);
    let (r_major, r_minor, r_patch) = parse_version_parts(req_ver);

    match prefix {
        "=" => a_major == r_major && a_minor == r_minor && a_patch == r_patch,
        ">=" => (a_major, a_minor, a_patch) >= (r_major, r_minor, r_patch),
        "~" => a_major == r_major && a_minor == r_minor && a_patch >= r_patch,
        _ => {
            if r_major > 0 {
                a_major == r_major && (a_minor, a_patch) >= (r_minor, r_patch)
            } else if r_minor > 0 {
                a_major == 0 && a_minor == r_minor && a_patch >= r_patch
            } else {
                a_major == 0 && a_minor == 0 && a_patch == r_patch
            }
        }
    }
}

fn parse_version_parts(version: &str) -> (u64, u64, u64) {
    let base = version.split('-').next().unwrap_or(version);
    let parts: Vec<&str> = base.split('.').collect();
    let major = parts.first().and_then(|p| p.parse().ok()).unwrap_or(0);
    let minor = if parts.len() > 1 {
        parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(0)
    } else {
        0
    };
    let patch = if parts.len() > 2 {
        parts.get(2).and_then(|p| p.parse().ok()).unwrap_or(0)
    } else {
        0
    };
    (major, minor, patch)
}

// --- R-PUB-06: keywords ---

pub fn check_keywords(
    pkg: Option<&toml::Value>,
    name: &str,
    file: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let keywords = pkg
        .and_then(|p| p.get("keywords"))
        .and_then(|k| k.as_array());

    let (severity, title, message) = match keywords {
        None => (
            Severity::Warn,
            format!("{name}: keywords missing"),
            "Cargo.toml [package].keywords is missing".to_owned(),
        ),
        Some(arr) if arr.is_empty() => (
            Severity::Warn,
            format!("{name}: keywords empty"),
            "Cargo.toml [package].keywords is empty".to_owned(),
        ),
        Some(arr) if arr.len() > 5 => (
            Severity::Warn,
            format!("{name}: too many keywords"),
            format!("{} keywords (max 5)", arr.len()),
        ),
        Some(arr) => (
            Severity::Info,
            format!("{name}: keywords present"),
            format!("{} keywords", arr.len()),
        ),
    };

    let result = CheckResult {
        id: "R-PUB-06".to_owned(),
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

// --- R-PUB-07: categories ---

pub fn check_categories(
    pkg: Option<&toml::Value>,
    name: &str,
    file: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let cats = pkg
        .and_then(|p| p.get("categories"))
        .and_then(|c| c.as_array());

    let is_missing_or_empty = match cats {
        None => true,
        Some(arr) => arr.is_empty(),
    };

    let result = CheckResult {
        id: "R-PUB-07".to_owned(),
        severity: if is_missing_or_empty {
            Severity::Warn
        } else {
            Severity::Info
        },
        title: if is_missing_or_empty {
            format!("{name}: categories missing")
        } else {
            format!("{name}: categories present")
        },
        message: if is_missing_or_empty {
            "Cargo.toml [package].categories is missing or empty".to_owned()
        } else {
            "Cargo.toml has [package].categories".to_owned()
        },
        file: file.map(std::borrow::ToOwned::to_owned),
        line: None,
        inventory: false,
    };
    results.push(if is_missing_or_empty {
        result
    } else {
        result.as_inventory()
    });
}

/// Check if a version string is valid semver (X.Y.Z with optional -prerelease).
pub fn is_valid_semver(version: &str) -> bool {
    let base = version.split('-').next().unwrap_or(version);
    let parts: Vec<&str> = base.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts
        .iter()
        .all(|p| !p.is_empty() && p.parse::<u64>().is_ok())
}
