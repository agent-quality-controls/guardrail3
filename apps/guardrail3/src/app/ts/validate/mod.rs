pub mod ast_helpers;
pub mod config_files;
pub mod eslint_audit;
mod eslint_check;
mod eslint_plugin_checks;
mod eslint_rule_infra;
pub mod i18n_check;
mod jscpd_check;
mod npmrc_check;
mod package_check;
mod package_deps;
pub mod source_scan;
mod stylelint_check;
pub mod test_checks;
mod tool_config_checks;
pub mod ts_arch_checks;
pub mod ts_code_analysis;
pub mod ts_comment_checks;
mod tsconfig_check;

use std::path::Path;

use crate::app::crawl::CrawlResult;
use crate::domain::config::types::GuardrailConfig;
use crate::domain::report::{Report, Section, TsAppContext, TsAppType, TsCheckCategories};
use crate::ports::outbound::FileSystem;

#[allow(clippy::too_many_lines)] // reason: TS validation orchestrator wires all check modules
pub fn run(
    fs: &dyn FileSystem,
    path: &Path,
    scoped_files: Option<&[String]>,
    categories: &TsCheckCategories,
    config: Option<&GuardrailConfig>,
    _crawl: &CrawlResult,
) -> Report {
    let mut report = Report::new(path.display().to_string(), vec!["TypeScript".to_owned()]);

    // Config file checks
    let config_results = config_files::check(fs, path);
    report.add_section(Section {
        name: "TS config files".to_owned(),
        results: config_results,
    });

    // ESLint plugin configuration (core — always run)
    let eslint_path = path.join("eslint.config.mjs");
    if let Some(eslint_content) = fs.read_file(&eslint_path) {
        let mut plugin_results = Vec::new();
        eslint_plugin_checks::check_core_plugins(
            &eslint_content,
            &eslint_path,
            &mut plugin_results,
        );
        report.add_section(Section {
            name: "ESLint plugin configuration".to_owned(),
            results: plugin_results,
        });
    }

    // Plugin packages in devDependencies
    let content_enabled = has_content_app(fs, path, config);
    let mut plug_results = Vec::new();
    package_deps::check_lint_plugins(fs, path, content_enabled, &mut plug_results);
    if !plug_results.is_empty() {
        report.add_section(Section {
            name: "Lint plugin packages".to_owned(),
            results: plug_results,
        });
    }

    // Additional tool packages (T-TOOL-01..06)
    let mut tool_pkg_results = Vec::new();
    package_deps::check_additional_tools(fs, path, content_enabled, &mut tool_pkg_results);
    if !tool_pkg_results.is_empty() {
        report.add_section(Section {
            name: "Additional tool packages".to_owned(),
            results: tool_pkg_results,
        });
    }

    // Tool configurations and scripts (T-TOOL-07..11)
    let mut tool_cfg_results = Vec::new();
    tool_config_checks::check_tool_configs(fs, path, content_enabled, &mut tool_cfg_results);
    if !tool_cfg_results.is_empty() {
        report.add_section(Section {
            name: "Tool configuration".to_owned(),
            results: tool_cfg_results,
        });
    }

    // Content-profile checks (only if project has content-type apps)
    if content_enabled {
        // ESLint content plugins (jsx-a11y, tailwind-ban)
        if let Some(eslint_content) = fs.read_file(&path.join("eslint.config.mjs")) {
            let mut content_plugin_results = Vec::new();
            eslint_plugin_checks::check_content_plugins(
                &eslint_content,
                &path.join("eslint.config.mjs"),
                &mut content_plugin_results,
            );
            report.add_section(Section {
                name: "Content profile: ESLint accessibility".to_owned(),
                results: content_plugin_results,
            });
        }

        // Stylelint (T-STYL-01..05)
        let mut styl_results = Vec::new();
        stylelint_check::check_stylelint(fs, path, &mut styl_results);
        report.add_section(Section {
            name: "Content profile: Stylelint + a11y".to_owned(),
            results: styl_results,
        });

        // i18n completeness (T-TOOL-12) — content profile
        let mut i18n_results = Vec::new();
        i18n_check::check_i18n(fs, path, &mut i18n_results);
        if !i18n_results.is_empty() {
            report.add_section(Section {
                name: "Content profile: i18n completeness".to_owned(),
                results: i18n_results,
            });
        }
    }

    // Source code scan (respects scope flags)
    let source_results = source_scan::check(fs, path, scoped_files);
    report.add_section(Section {
        name: "TS source code scan".to_owned(),
        results: source_results,
    });

    if categories.architecture {
        // Discover apps and resolve per-app context
        let app_contexts = resolve_app_contexts(fs, path, categories, config);

        // ESLint boundary audit (global, not per-app)
        let eslint_results = eslint_audit::check(fs, path);
        report.add_section(Section {
            name: "ESLint boundary audit".to_owned(),
            results: eslint_results,
        });

        // Per-app arch checks (only on service-type apps)
        let arch_structure = ts_arch_checks::check_hex_arch_structure_for_apps(fs, &app_contexts);
        let arch_imports = ts_arch_checks::check_import_boundaries_for_apps(fs, &app_contexts);
        let mut arch_results = arch_structure;
        arch_results.extend(arch_imports);
        if !arch_results.is_empty() {
            report.add_section(Section {
                name: "TS architecture".to_owned(),
                results: arch_results,
            });
        }
    }

    if categories.tests {
        // Test quality checks
        let test_results = test_checks::check(fs, path);
        report.add_section(Section {
            name: "TS test quality".to_owned(),
            results: test_results,
        });
    }

    report
}

/// Check if any app in the project is configured as content type,
/// or if the global content category is enabled, or if auto-detection
/// finds content signals in any discovered app.
fn has_content_app(fs: &dyn FileSystem, root: &Path, config: Option<&GuardrailConfig>) -> bool {
    // Check explicit config first
    if let Some(ts) = config.and_then(|c| c.typescript.as_ref()) {
        // Check global content setting
        if let Some(checks) = &ts.checks {
            if checks.content == Some(true) {
                return true;
            }
        }

        // Check per-app types
        if let Some(apps) = &ts.apps {
            for app_cfg in apps.values() {
                if let Some(t) = &app_cfg.type_ {
                    if t.eq_ignore_ascii_case("content") {
                        return true;
                    }
                }
            }
        }
    }

    // Auto-detect: scan discovered apps for content signals
    let discovered = ts_arch_checks::discover_ts_apps(fs, root);
    for app_path in &discovered {
        if auto_detect_app_type(fs, app_path) == Some(TsAppType::Content) {
            return true;
        }
    }

    false
}

/// Auto-detect app type from directory structure and package.json dependencies.
/// Returns None if no clear signal is found.
#[allow(clippy::disallowed_methods)] // reason: serde_json for per-app package.json inspection
pub fn auto_detect_app_type(fs: &dyn FileSystem, app_path: &Path) -> Option<TsAppType> {
    // Signal 1: hex arch structure → Service
    let has_modules_domain = app_path.join("src/modules/domain").is_dir();
    if has_modules_domain {
        return Some(TsAppType::Service);
    }

    // Signal 2: content directory at app root → Content
    let has_content_dir = app_path.join("content").is_dir();
    if has_content_dir {
        return Some(TsAppType::Content);
    }

    // Signal 3: check package.json dependencies
    let pkg_path = app_path.join("package.json");
    if let Some(content) = fs.read_file(&pkg_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            let deps = json.get("dependencies").and_then(|d| d.as_object());
            if let Some(deps) = deps {
                // Backend framework → Service
                let backend_frameworks = ["express", "fastify", "hono", "koa", "nestjs"];
                if backend_frameworks.iter().any(|f| deps.contains_key(*f)) {
                    return Some(TsAppType::Service);
                }

                // Content pipeline / SEO tool → Content
                // Only strong signals — remark/rehype/shiki/mdx are markdown
                // rendering libs that any app (including admin) could use
                let content_signals = [
                    "velite",
                    "contentlayer",
                    "nextra",
                    "next-seo",
                    "next-sitemap",
                ];
                if content_signals.iter().any(|s| deps.contains_key(*s)) {
                    return Some(TsAppType::Content);
                }

                // Next.js without hex arch or backend → likely Content
                // (services using Next.js would have src/modules/)
                if deps.contains_key("next") && !has_modules_domain {
                    return Some(TsAppType::Content);
                }
            }

            // Also check devDependencies for content signals (build tools like velite live here)
            let dev_deps = json.get("devDependencies").and_then(|d| d.as_object());
            if let Some(dev_deps) = dev_deps {
                let content_signals = [
                    "velite",
                    "contentlayer",
                    "nextra",
                    "next-seo",
                    "next-sitemap",
                ];
                if content_signals.iter().any(|s| dev_deps.contains_key(*s)) {
                    return Some(TsAppType::Content);
                }
            }
        }
    }

    // No clear signal → None (caller decides default)
    None
}

/// Discover TS apps and resolve per-app type and categories from config.
fn resolve_app_contexts(
    fs: &dyn FileSystem,
    root: &Path,
    _global_categories: &TsCheckCategories,
    config: Option<&GuardrailConfig>,
) -> Vec<TsAppContext> {
    let discovered = ts_arch_checks::discover_ts_apps(fs, root);
    let app_configs = config
        .and_then(|c| c.typescript.as_ref())
        .and_then(|t| t.apps.as_ref());

    discovered
        .into_iter()
        .map(|app_path| {
            let name = app_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_owned();

            // Look up per-app config
            let app_cfg = app_configs.and_then(|apps| apps.get(&name));

            // Resolve type: config > auto-detect > default (service)
            let app_type = app_cfg
                .and_then(|c| c.type_.as_deref())
                .map(TsAppType::from_str_or_default)
                .or_else(|| auto_detect_app_type(fs, &app_path))
                .unwrap_or(TsAppType::Service);

            // Resolve categories: type defaults > per-app overrides
            let type_defaults = app_type.default_categories();
            let categories = if let Some(checks) = app_cfg.and_then(|c| c.checks.as_ref()) {
                TsCheckCategories {
                    architecture: checks.architecture.unwrap_or(type_defaults.architecture),
                    content: checks.content.unwrap_or(type_defaults.content),
                    tests: checks.tests.unwrap_or(type_defaults.tests),
                }
            } else {
                type_defaults
            };

            TsAppContext {
                name,
                path: app_path,
                app_type,
                categories,
            }
        })
        .collect()
}
