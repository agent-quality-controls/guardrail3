use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

use super::eslint_parser;

pub fn check(fs: &dyn FileSystem, path: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    let eslint_path = path.join("eslint.config.mjs");
    if !eslint_path.exists() {
        // T1 already reports this — skip silently
        return results;
    }

    let Some(content) = fs.read_file(&eslint_path) else {
        return results;
    };

    // Parse once with tree-sitter
    let config = eslint_parser::parse_eslint_config(&content)
        .unwrap_or_else(|| eslint_parser::EslintConfig::fallback(content));

    check_zone_definitions(&config, &eslint_path, &mut results);
    check_import_direction(&config, &eslint_path, &mut results);
    check_entry_point(&config, &eslint_path, &mut results);
    check_external_deps(&config, &eslint_path, &mut results);

    results
}

/// T36: Zone definitions
fn check_zone_definitions(
    config: &eslint_parser::EslintConfig,
    eslint_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    // Check for element-types rule in parsed rules, or domain+commands/adapters in raw content
    let has_zones = config.rules.contains_key("boundaries/element-types")
        || (config.raw_content.contains("element-types"))
        || (config.raw_content.contains("domain")
            && (config.raw_content.contains("commands")
                || config.raw_content.contains("adapters")));

    if has_zones {
        results.push(CheckResult {
            id: "T36".to_owned(),
            severity: Severity::Info,
            title: "Boundary zone definitions configured".to_owned(),
            message: "Zone definitions (element-types, domain/adapters) found in ESLint boundaries config. \
                     Zones define architectural layers so the boundaries plugin can enforce import direction rules."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T36".to_owned(),
            severity: Severity::Error,
            title: "No boundary zone definitions".to_owned(),
            message: "No boundary zone definitions found in ESLint config. Zones define architectural layers \
                     (domain, ports, adapters, application) so the boundaries plugin can enforce import direction. \
                     Without zones, boundaries enforcement has no layers to protect. Add `element-types` \
                     configuration to `eslint.config.mjs`."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T37: Import direction rules
fn check_import_direction(
    config: &eslint_parser::EslintConfig,
    eslint_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    if config.rules.contains_key("boundaries/element-types") {
        results.push(CheckResult {
            id: "T37".to_owned(),
            severity: Severity::Info,
            title: "Import direction rules configured".to_owned(),
            message: "`boundaries/element-types` rule found. This enforces that imports flow inward \
                     (adapters -> application -> domain), preventing domain from depending on infrastructure."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T37".to_owned(),
            severity: Severity::Error,
            title: "No import direction rules configured".to_owned(),
            message: "`boundaries/element-types` not found in ESLint config. This rule enforces that imports \
                     flow inward (adapters -> application -> domain), preventing domain code from depending on \
                     infrastructure. Add `boundaries/element-types` with allowed import directions to \
                     `eslint.config.mjs`."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T38: Entry-point barrel enforcement
fn check_entry_point(
    config: &eslint_parser::EslintConfig,
    eslint_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    if config.rules.contains_key("boundaries/entry-point") {
        results.push(CheckResult {
            id: "T38".to_owned(),
            severity: Severity::Info,
            title: "Entry-point barrel enforcement configured".to_owned(),
            message: "`boundaries/entry-point` found. This ensures modules are only imported through their \
                     public barrel files (index.ts), preventing deep imports into internal implementation."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T38".to_owned(),
            severity: Severity::Error,
            title: "No entry-point barrel enforcement".to_owned(),
            message: "`boundaries/entry-point` not found in ESLint config. Without this, any file can import \
                     internal implementation details of any module, creating tight coupling. Add \
                     `boundaries/entry-point` to enforce imports through barrel files (index.ts) only."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T39: External dependency per-zone bans
fn check_external_deps(
    config: &eslint_parser::EslintConfig,
    eslint_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    if config.rules.contains_key("boundaries/external") {
        results.push(CheckResult {
            id: "T39".to_owned(),
            severity: Severity::Info,
            title: "External dependency per-zone bans configured".to_owned(),
            message: "`boundaries/external` found. This restricts which external packages each architectural \
                     zone can import (e.g., domain cannot import database drivers)."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T39".to_owned(),
            severity: Severity::Error,
            title: "No external dependency per-zone bans".to_owned(),
            message: "`boundaries/external` not found in ESLint config. Without this, any layer can import \
                     any npm package — domain code could import database drivers or HTTP clients directly. \
                     Add `boundaries/external` rules to restrict which external packages each zone can use."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}
