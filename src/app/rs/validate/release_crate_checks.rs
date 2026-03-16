use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

use super::release_checks::CrateInfo;
use super::release_crate_deps;
use crate::ports::outbound::{FileSystem, ToolChecker};

/// Run all per-crate checks on a single publishable crate.
pub fn check_per_crate(fs: &dyn FileSystem, tc: &dyn ToolChecker,
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
fn check_required_string_field(
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

fn check_license(
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

fn check_readme(fs: &dyn FileSystem, 
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

fn check_readme_quality(fs: &dyn FileSystem, 
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

fn check_version(
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_toml(content: &str) -> Option<toml::Value> {
        content.parse().ok()
    }

    fn pkg_from(table: &toml::Value) -> Option<&toml::Value> {
        table.get("package")
    }

    // --- R-PUB-01 ---

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn pub01_neg_no_description() {
        let t = parse_toml("[package]\nname = \"x\"\nversion = \"0.1.0\"").expect("parse"); // reason: test
        let mut r = Vec::new();
        check_required_string_field(pkg_from(&t), "description", "R-PUB-01", "x", None, &mut r);
        assert_eq!(r.len(), 1, "expected one result");
        assert_eq!(
            r.first().map(|c| c.severity),
            Some(Severity::Error),
            "expected Error"
        );
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn pub01_pos_has_description() {
        let t = parse_toml("[package]\nname = \"x\"\ndescription = \"A crate\"").expect("parse"); // reason: test
        let mut r = Vec::new();
        check_required_string_field(pkg_from(&t), "description", "R-PUB-01", "x", None, &mut r);
        assert_eq!(r.len(), 1, "expected one result");
        assert_eq!(
            r.first().map(|c| c.severity),
            Some(Severity::Info),
            "expected Info"
        );
    }

    // --- R-PUB-02 ---

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn pub02_neg_no_license() {
        let t = parse_toml("[package]\nname = \"x\"").expect("parse"); // reason: test
        let mut r = Vec::new();
        check_license(pkg_from(&t), "x", None, &mut r);
        assert_eq!(
            r.first().map(|c| c.severity),
            Some(Severity::Error),
            "expected Error"
        );
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn pub02_pos_license() {
        let t = parse_toml("[package]\nname = \"x\"\nlicense = \"MIT\"").expect("parse"); // reason: test
        let mut r = Vec::new();
        check_license(pkg_from(&t), "x", None, &mut r);
        assert_eq!(
            r.first().map(|c| c.severity),
            Some(Severity::Info),
            "expected Info"
        );
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn pub02_pos_alt_license_file() {
        let t = parse_toml("[package]\nname = \"x\"\nlicense-file = \"LICENSE\"").expect("parse"); // reason: test
        let mut r = Vec::new();
        check_license(pkg_from(&t), "x", None, &mut r);
        assert_eq!(
            r.first().map(|c| c.severity),
            Some(Severity::Info),
            "expected Info"
        );
    }

    // --- R-PUB-03 ---

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn pub03_neg_no_repository() {
        let t = parse_toml("[package]\nname = \"x\"").expect("parse"); // reason: test
        let mut r = Vec::new();
        check_required_string_field(pkg_from(&t), "repository", "R-PUB-03", "x", None, &mut r);
        assert_eq!(
            r.first().map(|c| c.severity),
            Some(Severity::Error),
            "expected Error"
        );
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn pub03_pos_has_repository() {
        let t = parse_toml("[package]\nname = \"x\"\nrepository = \"https://github.com/x/x\"")
            .expect("parse"); // reason: test
        let mut r = Vec::new();
        check_required_string_field(pkg_from(&t), "repository", "R-PUB-03", "x", None, &mut r);
        assert_eq!(
            r.first().map(|c| c.severity),
            Some(Severity::Info),
            "expected Info"
        );
    }

    // --- R-PUB-04 ---

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn pub04_neg_no_field_no_file() {
        let fs = crate::adapters::outbound::fs::RealFileSystem;
        let tmp = std::env::temp_dir().join("guardrail3_pub04_neg");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);
        let t = parse_toml("[package]\nname = \"x\"").expect("parse"); // reason: test
        let krate = CrateInfo {
            name: "x".to_owned(),
            cargo_toml_path: tmp.join("Cargo.toml"),
            dir: tmp.clone(),
            publishable: true,
            is_binary: false,
            table: t.clone(),
        };
        let mut r = Vec::new();
        check_readme(&fs, pkg_from(&t), &krate, None, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-PUB-04" && c.severity == Severity::Warn),
            "expected Warn"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn pub04_neg_dangling_readme() {
        let fs = crate::adapters::outbound::fs::RealFileSystem;
        let tmp = std::env::temp_dir().join("guardrail3_pub04_dangle");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);
        let t = parse_toml("[package]\nname = \"x\"\nreadme = \"MISSING.md\"").expect("parse"); // reason: test
        let krate = CrateInfo {
            name: "x".to_owned(),
            cargo_toml_path: tmp.join("Cargo.toml"),
            dir: tmp.clone(),
            publishable: true,
            is_binary: false,
            table: t.clone(),
        };
        let mut r = Vec::new();
        check_readme(&fs, pkg_from(&t), &krate, None, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-PUB-04" && c.severity == Severity::Warn),
            "expected Warn"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn pub04_pos_readme_exists() {
        let fs = crate::adapters::outbound::fs::RealFileSystem;
        let tmp = std::env::temp_dir().join("guardrail3_pub04_pos");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);
        let _ = std::fs::write(
            tmp.join("README.md"),
            format!("# My Crate\n\n{}", "x".repeat(200)),
        );
        let t = parse_toml("[package]\nname = \"x\"\nreadme = \"README.md\"").expect("parse"); // reason: test
        let krate = CrateInfo {
            name: "x".to_owned(),
            cargo_toml_path: tmp.join("Cargo.toml"),
            dir: tmp.clone(),
            publishable: true,
            is_binary: false,
            table: t.clone(),
        };
        let mut r = Vec::new();
        check_readme(&fs, pkg_from(&t), &krate, None, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-PUB-04" && c.severity == Severity::Info),
            "expected Info"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- R-PUB-05 ---

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn pub05_neg_small_readme() {
        let fs = crate::adapters::outbound::fs::RealFileSystem;
        let tmp = std::env::temp_dir().join("guardrail3_pub05_small");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);
        let _ = std::fs::write(tmp.join("README.md"), "# Hi\nshort");
        let mut r = Vec::new();
        check_readme_quality(&fs, &tmp.join("README.md"), "x", None, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-PUB-05" && c.severity == Severity::Warn),
            "expected Warn"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn pub05_neg_no_heading() {
        let fs = crate::adapters::outbound::fs::RealFileSystem;
        let tmp = std::env::temp_dir().join("guardrail3_pub05_nohead");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);
        let _ = std::fs::write(tmp.join("README.md"), "x".repeat(300));
        let mut r = Vec::new();
        check_readme_quality(&fs, &tmp.join("README.md"), "x", None, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-PUB-05" && c.severity == Severity::Warn),
            "expected Warn"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn pub05_pos_good_readme() {
        let fs = crate::adapters::outbound::fs::RealFileSystem;
        let tmp = std::env::temp_dir().join("guardrail3_pub05_good");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);
        let _ = std::fs::write(
            tmp.join("README.md"),
            format!("# My Crate\n\n{}", "content ".repeat(50)),
        );
        let mut r = Vec::new();
        check_readme_quality(&fs, &tmp.join("README.md"), "x", None, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-PUB-05" && c.severity == Severity::Info),
            "expected Info"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- R-PUB-08 ---

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn pub08_neg_invalid() {
        let t = parse_toml("[package]\nname = \"x\"\nversion = \"not-a-version\"").expect("parse"); // reason: test
        let mut r = Vec::new();
        check_version(pkg_from(&t), "x", None, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-PUB-08" && c.severity == Severity::Error),
            "expected Error"
        );
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn pub08_pos_valid() {
        let t = parse_toml("[package]\nname = \"x\"\nversion = \"1.2.3\"").expect("parse"); // reason: test
        let mut r = Vec::new();
        check_version(pkg_from(&t), "x", None, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-PUB-08" && c.severity == Severity::Info),
            "expected Info"
        );
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn pub08_pos_prerelease() {
        let t = parse_toml("[package]\nname = \"x\"\nversion = \"1.2.3-beta.1\"").expect("parse"); // reason: test
        let mut r = Vec::new();
        check_version(pkg_from(&t), "x", None, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-PUB-08" && c.severity == Severity::Info),
            "expected Info"
        );
    }
}
