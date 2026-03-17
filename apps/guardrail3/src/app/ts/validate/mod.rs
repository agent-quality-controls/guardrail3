pub mod ast_helpers;
pub mod config_files;
pub mod eslint_audit;
mod eslint_check;
mod eslint_rule_infra;
mod jscpd_check;
mod npmrc_check;
mod package_check;
pub mod source_scan;
pub mod test_checks;
pub mod ts_arch_checks;
pub mod ts_code_analysis;
pub mod ts_comment_checks;
mod tsconfig_check;

use std::path::Path;

use crate::domain::config::types::GuardrailConfig;
use crate::domain::report::{Report, Section, TsAppContext, TsAppType, TsCheckCategories};
use crate::ports::outbound::FileSystem;

pub fn run(
    fs: &dyn FileSystem,
    path: &Path,
    scoped_files: Option<&[String]>,
    categories: &TsCheckCategories,
    config: Option<&GuardrailConfig>,
) -> Report {
    let mut report = Report::new(path.display().to_string(), vec!["TypeScript".to_owned()]);

    // Config file checks
    let config_results = config_files::check(fs, path);
    report.add_section(Section {
        name: "TS config files".to_owned(),
        results: config_results,
    });

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

            // Resolve type: config > default (service)
            let app_type = app_cfg
                .and_then(|c| c.type_.as_deref())
                .map_or(TsAppType::Service, TsAppType::from_str_or_default);

            // Resolve categories: type defaults > per-app overrides
            let type_defaults = app_type.default_categories();
            let categories = if let Some(checks) = app_cfg.and_then(|c| c.checks.as_ref()) {
                TsCheckCategories {
                    architecture: checks.architecture.unwrap_or(type_defaults.architecture),
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
