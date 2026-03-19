//! Package dependency checks — lint plugins and tool packages in devDependencies.

use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

type JsonMap = serde_json::Map<String, serde_json::Value>;

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

/// Check lint plugin packages in devDependencies (T-PLUG-*).
#[allow(clippy::disallowed_methods)] // reason: serde_json::from_str for package.json inspection
#[allow(clippy::too_many_lines)] // reason: checks 19 packages + knip script sequentially — splitting would obscure the check list
pub fn check_lint_plugins(
    fs: &dyn FileSystem,
    path: &Path,
    content_enabled: bool,
    results: &mut Vec<CheckResult>,
) {
    let pkg_path = path.join("package.json");
    let Some(content) = fs.read_file(&pkg_path) else {
        return;
    };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) else {
        return;
    };

    let dev_deps = json.get("devDependencies").and_then(|d| d.as_object());

    // Core plugins (always)
    check_dev_dep(
        "T-PLUG-01",
        "eslint-plugin-unicorn",
        dev_deps,
        &pkg_path,
        results,
    );
    check_dev_dep(
        "T-PLUG-02",
        "eslint-plugin-regexp",
        dev_deps,
        &pkg_path,
        results,
    );
    check_dev_dep(
        "T-PLUG-03",
        "eslint-plugin-sonarjs",
        dev_deps,
        &pkg_path,
        results,
    );
    check_dev_dep("T-PLUG-10", "knip", dev_deps, &pkg_path, results);
    check_dev_dep("T-PLUG-12", "eslint", dev_deps, &pkg_path, results);
    check_dev_dep("T-PLUG-13", "typescript", dev_deps, &pkg_path, results);
    check_dev_dep(
        "T-PLUG-14",
        "typescript-eslint",
        dev_deps,
        &pkg_path,
        results,
    );
    check_dev_dep(
        "T-PLUG-15",
        "eslint-plugin-import-x",
        dev_deps,
        &pkg_path,
        results,
    );
    check_dev_dep(
        "T-PLUG-16",
        "eslint-import-resolver-typescript",
        dev_deps,
        &pkg_path,
        results,
    );
    check_dev_dep(
        "T-PLUG-17",
        "eslint-plugin-boundaries",
        dev_deps,
        &pkg_path,
        results,
    );
    check_dev_dep("T-PLUG-18", "only-allow", dev_deps, &pkg_path, results);
    check_dev_dep("T-PLUG-19", "jscpd", dev_deps, &pkg_path, results);

    // Content-profile plugins
    if content_enabled {
        check_dev_dep(
            "T-PLUG-04",
            "eslint-plugin-jsx-a11y",
            dev_deps,
            &pkg_path,
            results,
        );
        check_dev_dep("T-PLUG-05", "stylelint", dev_deps, &pkg_path, results);
        check_dev_dep(
            "T-PLUG-06",
            "@double-great/stylelint-a11y",
            dev_deps,
            &pkg_path,
            results,
        );
        check_dev_dep(
            "T-PLUG-07",
            "stylelint-config-standard",
            dev_deps,
            &pkg_path,
            results,
        );
        check_dev_dep(
            "T-PLUG-08",
            "stylelint-config-tailwindcss",
            dev_deps,
            &pkg_path,
            results,
        );
        check_dev_dep(
            "T-PLUG-09",
            "eslint-plugin-tailwind-ban",
            dev_deps,
            &pkg_path,
            results,
        );
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
#[allow(clippy::disallowed_methods)] // reason: serde_json::from_str for package.json inspection
pub fn check_additional_tools(
    fs: &dyn FileSystem,
    path: &Path,
    content_enabled: bool,
    results: &mut Vec<CheckResult>,
) {
    let pkg_path = path.join("package.json");
    let Some(content) = fs.read_file(&pkg_path) else {
        return;
    };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) else {
        return;
    };

    let dev_deps = json.get("devDependencies").and_then(|d| d.as_object());

    // Core tools (always)
    check_dev_dep("T-TOOL-01", "cspell", dev_deps, &pkg_path, results);
    check_dev_dep("T-TOOL-02", "type-coverage", dev_deps, &pkg_path, results);
    check_dev_dep("T-TOOL-03", "license-checker", dev_deps, &pkg_path, results);
    check_dev_dep("T-TOOL-04", "prettier", dev_deps, &pkg_path, results);

    // Content-profile tools
    if content_enabled {
        check_dev_dep("T-TOOL-05", "size-limit", dev_deps, &pkg_path, results);
        check_dev_dep(
            "T-TOOL-06",
            "@size-limit/preset-app",
            dev_deps,
            &pkg_path,
            results,
        );
    }
}
