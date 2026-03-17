use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

#[allow(clippy::too_many_lines, clippy::disallowed_methods)] // reason: comprehensive package.json validation; guardrail3 JSON config inspection
pub fn check_package_json(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let pkg_path = path.join("package.json");
    if !pkg_path.exists() {
        return;
    }

    let Some(content) = fs.read_file(&pkg_path) else {
        return;
    };

    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return,
    };

    // T15: pnpm.overrides
    let overrides = json.get("pnpm").and_then(|p| p.get("overrides"));
    match overrides {
        Some(ov) if ov.is_object() => {
            let Some(ov_obj) = ov.as_object() else {
                return;
            };
            let has_zod = ov_obj.contains_key("zod");
            let has_eslint_js = ov_obj.contains_key("@eslint/js");

            if !has_zod {
                results.push(CheckResult {
                    id: "T15".to_owned(),
                    severity: Severity::Error,
                    title: "`pnpm.overrides` missing `zod` pin".to_owned(),
                    message: "No `zod` override in `pnpm.overrides`. Overrides pin transitive dependency \
                             versions to a single copy, preventing version conflicts and reducing bundle size. \
                             Add `\"zod\": \"<version>\"` to `pnpm.overrides` in `package.json`."
                        .to_owned(),
                    file: Some(pkg_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
            if !has_eslint_js {
                results.push(CheckResult {
                    id: "T15".to_owned(),
                    severity: Severity::Error,
                    title: "`pnpm.overrides` missing `@eslint/js` pin".to_owned(),
                    message: "No `@eslint/js` override in `pnpm.overrides`. Overrides pin transitive dependency \
                             versions to a single copy. Add `\"@eslint/js\": \"<version>\"` to `pnpm.overrides` \
                             in `package.json`."
                        .to_owned(),
                    file: Some(pkg_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }

            // T16: Extra overrides
            let known_overrides = ["zod", "@eslint/js"];
            for key in ov_obj.keys() {
                if !known_overrides.contains(&key.as_str()) {
                    results.push(CheckResult {
                        id: "T16".to_owned(),
                        severity: Severity::Info,
                        title: format!("Extra pnpm override: `{key}`"),
                        message: format!(
                            "Non-baseline pnpm override `{key}` = {}. Extra overrides pin transitive \
                             dependency versions. Verify this is intentional and the pinned version is current.",
                            ov_obj
                                .get(key)
                                .map_or_else(|| "?".to_owned(), std::string::ToString::to_string)
                        ),
                        file: Some(pkg_path.display().to_string()),
                        line: None,
                        inventory: false,
                    }.as_inventory());
                }
            }
        }
        _ => {
            results.push(CheckResult {
                id: "T15".to_owned(),
                severity: Severity::Error,
                title: "`pnpm.overrides` section missing from `package.json`".to_owned(),
                message: "No `pnpm.overrides` section in `package.json`. Overrides pin transitive dependency \
                         versions to prevent duplicate packages and version conflicts. Add a `pnpm.overrides` \
                         section with at least `zod` and `@eslint/js` pinned to workspace versions."
                    .to_owned(),
                file: Some(pkg_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    // T17: Banned dependencies
    let banned_deps: &[&str] = &[
        "axios",
        "lodash",
        "moment",
        "uuid",
        "nanoid",
        "pg",
        "express",
        "classnames",
        "winston",
        "pino",
        "request",
        "got",
        "superagent",
        "node-fetch",
        "isomorphic-fetch",
        "underscore",
        "request-promise",
        "postgres",
        "cross-fetch",
    ];
    let banned_prefixes: &[&str] = &["embla-carousel"];

    for section_name in &["dependencies", "devDependencies"] {
        if let Some(deps) = json.get(section_name).and_then(|d| d.as_object()) {
            for dep_name in deps.keys() {
                let is_banned = banned_deps.contains(&dep_name.as_str())
                    || banned_prefixes.iter().any(|p| dep_name.starts_with(p));

                if is_banned {
                    results.push(CheckResult {
                        id: "T17".to_owned(),
                        severity: Severity::Error,
                        title: format!("Banned dependency `{dep_name}` in `{section_name}`"),
                        message: format!(
                            "`{dep_name}` found in `{section_name}`. This package is banned because a preferred \
                             alternative exists (e.g., native fetch over axios, date-fns over moment, \
                             crypto.randomUUID over uuid). Remove it and switch to the approved alternative."
                        ),
                        file: Some(pkg_path.display().to_string()),
                        line: None,
                        inventory: false,
                    });
                }
            }
        }
    }

    // T18: packageManager field
    if json.get("packageManager").is_some() {
        results.push(CheckResult {
            id: "T18".to_owned(),
            severity: Severity::Info,
            title: "`packageManager` field set in `package.json`".to_owned(),
            message: format!(
                "`packageManager` = {}. This field pins the package manager version via corepack, \
                 ensuring all developers and CI use the same pnpm version.",
                json.get("packageManager")
                    .map_or_else(|| "?".to_owned(), std::string::ToString::to_string)
            ),
            file: Some(pkg_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T18".to_owned(),
            severity: Severity::Warn,
            title: "`packageManager` field missing from `package.json`".to_owned(),
            message: "No `packageManager` field in `package.json`. Without this, developers may use different \
                     pnpm versions, causing lockfile conflicts and inconsistent behavior. Add \
                     `\"packageManager\": \"pnpm@<version>\"` to `package.json` and enable corepack."
                .to_owned(),
            file: Some(pkg_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // T55: preinstall script contains only-allow pnpm
    let preinstall = json
        .get("scripts")
        .and_then(|s| s.get("preinstall"))
        .and_then(|v| v.as_str());

    match preinstall {
        Some(script) if script.contains("only-allow pnpm") => {
            results.push(CheckResult {
                id: "T55".to_owned(),
                severity: Severity::Info,
                title: "`preinstall` script enforces pnpm".to_owned(),
                message: "`preinstall` script contains `only-allow pnpm`. This prevents accidentally running \
                         `npm install` or `yarn install`, which would create a conflicting lockfile."
                    .to_owned(),
                file: Some(pkg_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        }
        _ => {
            results.push(CheckResult {
                id: "T55".to_owned(),
                severity: Severity::Warn,
                title: "`preinstall` script missing pnpm enforcement".to_owned(),
                message: "No `preinstall` script with `only-allow pnpm`. Without this, running `npm install` \
                         or `yarn install` would create a conflicting lockfile. Add \
                         `\"preinstall\": \"npx only-allow pnpm\"` to scripts in `package.json`."
                    .to_owned(),
                file: Some(pkg_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    // T56: prepare script exists
    let prepare = json.get("scripts").and_then(|s| s.get("prepare"));

    if prepare.is_some() {
        results.push(
            CheckResult {
                id: "T56".to_owned(),
                severity: Severity::Info,
                title: "`prepare` script exists in `package.json`".to_owned(),
                message:
                    "`prepare` script found. This runs after `pnpm install`, typically setting up \
                     git hooks (e.g., husky) or building required artifacts."
                        .to_owned(),
                file: Some(pkg_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "T56".to_owned(),
            severity: Severity::Warn,
            title: "`prepare` script missing from `package.json`".to_owned(),
            message: "No `prepare` script in `package.json`. The `prepare` script runs after `pnpm install` \
                     and is typically used to set up git hooks (e.g., `\"prepare\": \"husky\"`). Without it, \
                     new developers won't get pre-commit hooks installed automatically."
                .to_owned(),
            file: Some(pkg_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // T57: engines field
    if json.get("engines").is_some() {
        results.push(CheckResult {
            id: "T57".to_owned(),
            severity: Severity::Info,
            title: "`engines` field set in `package.json`".to_owned(),
            message: format!(
                "`engines` = {}. This specifies the minimum Node.js version required, preventing \
                 deployment to incompatible runtimes.",
                json.get("engines")
                    .map_or_else(|| "?".to_owned(), std::string::ToString::to_string)
            ),
            file: Some(pkg_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T57".to_owned(),
            severity: Severity::Warn,
            title: "`engines` field missing from `package.json`".to_owned(),
            message: "No `engines` field in `package.json`. Without this, the project may be deployed to an \
                     incompatible Node.js version. Add `\"engines\": { \"node\": \">=20\" }` (or your minimum \
                     supported version) to `package.json`."
                .to_owned(),
            file: Some(pkg_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // T58: onlyBuiltDependencies
    if let Some(obd) = json
        .get("pnpm")
        .and_then(|p| p.get("onlyBuiltDependencies"))
    {
        results.push(CheckResult {
            id: "T58".to_owned(),
            severity: Severity::Info,
            title: "`onlyBuiltDependencies` configured in pnpm".to_owned(),
            message: format!(
                "`onlyBuiltDependencies` = {obd}. This restricts which packages can run post-install scripts, \
                 reducing supply chain attack surface by blocking arbitrary code execution from dependencies."
            ),
            file: Some(pkg_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    }
}

#[allow(clippy::disallowed_methods)] // reason: serde_json::from_str for package.json inspection
#[allow(clippy::too_many_lines)] // reason: checks 11 packages + script sequentially
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

    let check_pkg = |id: &str, pkg: &str, out: &mut Vec<CheckResult>| {
        let found = dev_deps.is_some_and(|d| d.contains_key(pkg));
        if found {
            out.push(
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
            out.push(CheckResult {
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
    };

    // Core plugins (always)
    check_pkg("T-PLUG-01", "eslint-plugin-unicorn", results);
    check_pkg("T-PLUG-02", "eslint-plugin-regexp", results);
    check_pkg("T-PLUG-03", "eslint-plugin-sonarjs", results);
    check_pkg("T-PLUG-10", "knip", results);

    // Content-profile plugins
    if content_enabled {
        check_pkg("T-PLUG-04", "eslint-plugin-jsx-a11y", results);
        check_pkg("T-PLUG-05", "stylelint", results);
        check_pkg("T-PLUG-06", "@double-great/stylelint-a11y", results);
        check_pkg("T-PLUG-07", "stylelint-config-standard", results);
        check_pkg("T-PLUG-08", "stylelint-config-tailwindcss", results);
        check_pkg("T-PLUG-09", "eslint-plugin-tailwind-ban", results);
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
            severity: Severity::Warn,
            title: "knip script missing".to_owned(),
            message: "No \"knip\" script in package.json. Add `\"knip\": \"knip\"` to scripts for dead code detection.".to_owned(),
            file: Some(pkg_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// Check additional tool packages in devDependencies.
#[allow(clippy::disallowed_methods)] // reason: serde_json::from_str for package.json inspection
#[allow(clippy::too_many_lines)] // reason: checks multiple tools sequentially
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

    let check_pkg = |id: &str, pkg: &str, out: &mut Vec<CheckResult>| {
        let found = dev_deps.is_some_and(|d| d.contains_key(pkg));
        if found {
            out.push(
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
            out.push(CheckResult {
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
    };

    // Core tools (always)
    check_pkg("T-TOOL-01", "cspell", results);
    check_pkg("T-TOOL-02", "type-coverage", results);
    check_pkg("T-TOOL-03", "license-checker", results);
    check_pkg("T-TOOL-04", "prettier", results);

    // Content-profile tools
    if content_enabled {
        check_pkg("T-TOOL-05", "size-limit", results);
        check_pkg("T-TOOL-06", "@size-limit/preset-app", results);
    }
}
