//! Package dependency checks — lint plugins and tool packages in devDependencies.

use std::path::{Path, PathBuf};

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

type JsonMap = serde_json::Map<String, serde_json::Value>;
type PackageJson = (PathBuf, serde_json::Value);

const CORE_LINT_PLUGINS: &[(&str, &str)] = &[
    ("T-PLUG-01", "eslint-plugin-unicorn"),
    ("T-PLUG-02", "eslint-plugin-regexp"),
    ("T-PLUG-03", "eslint-plugin-sonarjs"),
    ("T-PLUG-10", "knip"),
    ("T-PLUG-12", "eslint"),
    ("T-PLUG-13", "typescript"),
    ("T-PLUG-14", "typescript-eslint"),
    ("T-PLUG-15", "eslint-plugin-import-x"),
    ("T-PLUG-16", "eslint-import-resolver-typescript"),
    ("T-PLUG-17", "eslint-plugin-boundaries"),
    ("T-PLUG-18", "only-allow"),
    ("T-PLUG-19", "jscpd"),
];

const CONTENT_LINT_PLUGINS: &[(&str, &str)] = &[
    ("T-PLUG-04", "eslint-plugin-jsx-a11y"),
    ("T-PLUG-05", "stylelint"),
    ("T-PLUG-06", "@double-great/stylelint-a11y"),
    ("T-PLUG-07", "stylelint-config-standard"),
    ("T-PLUG-08", "stylelint-config-tailwindcss"),
    ("T-PLUG-09", "eslint-plugin-tailwind-ban"),
];

const CORE_TOOLS: &[(&str, &str)] = &[
    ("T-TOOL-01", "cspell"),
    ("T-TOOL-02", "type-coverage"),
    ("T-TOOL-03", "license-checker"),
    ("T-TOOL-04", "prettier"),
];

const CONTENT_TOOLS: &[(&str, &str)] = &[
    ("T-TOOL-05", "size-limit"),
    ("T-TOOL-06", "@size-limit/preset-app"),
];

/// Check a single devDependency. Error if missing, Info inventory if present.
fn check_dev_dep(
    id: &str,
    pkg: &str,
    dev_deps: Option<&JsonMap>,
    pkg_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let found = dev_deps.is_some_and(|d| d.contains_key(pkg));
    if found {
        results.push(
            CheckResult {
                id: id.to_owned(),
                severity: Severity::Info,
                title: format!("{pkg} installed"),
                message: format!("{pkg} found in devDependencies."),
                file: Some(pkg_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: id.to_owned(),
            severity: Severity::Error,
            title: format!("{pkg} missing"),
            message: format!(
                "{pkg} not found in devDependencies. Install with: pnpm add -Dw {pkg}"
            ),
            file: Some(pkg_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

fn load_root_package_json(
    fs: &dyn FileSystem,
    package_jsons: &[PathBuf],
    root: &Path,
) -> Option<PackageJson> {
    let pkg_path = package_jsons
        .iter()
        .find(|path| path.parent().is_some_and(|parent| parent == root))?;
    let content = fs.read_file(pkg_path)?;
    let json = serde_json::from_str::<serde_json::Value>(&content).ok()?;
    Some((pkg_path.clone(), json))
}

fn check_dev_dep_group(
    deps: &[(&str, &str)],
    dev_deps: Option<&JsonMap>,
    pkg_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    for &(id, pkg) in deps {
        check_dev_dep(id, pkg, dev_deps, pkg_path, results);
    }
}

/// Check lint plugin packages in devDependencies (T-PLUG-*).
pub fn check_lint_plugins(
    fs: &dyn FileSystem,
    package_jsons: &[PathBuf],
    root: &Path,
    content_enabled: bool,
    results: &mut Vec<CheckResult>,
) {
    let Some((pkg_path, json)) = load_root_package_json(fs, package_jsons, root) else {
        return;
    };

    let dev_deps = json.get("devDependencies").and_then(|d| d.as_object());

    check_dev_dep_group(CORE_LINT_PLUGINS, dev_deps, &pkg_path, results);

    if content_enabled {
        check_dev_dep_group(CONTENT_LINT_PLUGINS, dev_deps, &pkg_path, results);
    }

    // T-PLUG-11: knip script in package.json
    let has_knip_script = json
        .get("scripts")
        .and_then(|s| s.as_object())
        .is_some_and(|scripts| scripts.contains_key("knip"));
    if has_knip_script {
        results.push(
            CheckResult {
                id: "T-PLUG-11".to_owned(),
                severity: Severity::Info,
                title: "knip script configured".to_owned(),
                message: "\"knip\" script found in package.json scripts.".to_owned(),
                file: Some(pkg_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "T-PLUG-11".to_owned(),
            severity: Severity::Error,
            title: "knip script missing".to_owned(),
            message: "No \"knip\" script in package.json. Add `\"knip\": \"knip\"` to scripts \
                     for dead code detection."
                .to_owned(),
            file: Some(pkg_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// Check additional tool packages in devDependencies (T-TOOL-01..06).
pub fn check_additional_tools(
    fs: &dyn FileSystem,
    package_jsons: &[PathBuf],
    root: &Path,
    content_enabled: bool,
    results: &mut Vec<CheckResult>,
) {
    let Some((pkg_path, json)) = load_root_package_json(fs, package_jsons, root) else {
        return;
    };

    let dev_deps = json.get("devDependencies").and_then(|d| d.as_object());

    check_dev_dep_group(CORE_TOOLS, dev_deps, &pkg_path, results);

    if content_enabled {
        check_dev_dep_group(CONTENT_TOOLS, dev_deps, &pkg_path, results);
    }
}
