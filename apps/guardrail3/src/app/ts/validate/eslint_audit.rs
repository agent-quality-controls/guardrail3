use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

#[allow(clippy::too_many_lines)] // reason: ESLint audit analysis
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

    // T36: Zone definitions
    let has_zones = content.contains("element-types")
        || content.contains("domain")
            && (content.contains("commands") || content.contains("adapters"));

    if has_zones {
        results.push(CheckResult {
            id: "T36".to_owned(),
            severity: Severity::Info,
            title: "Boundary zones configured".to_owned(),
            message: "Zone definitions found in ESLint config".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T36".to_owned(),
            severity: Severity::Error,
            title: "No boundary zones".to_owned(),
            message: "No boundary zone definitions found in ESLint config".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // T37: Import direction rules
    if content.contains("boundaries/element-types") {
        results.push(CheckResult {
            id: "T37".to_owned(),
            severity: Severity::Info,
            title: "Import direction rules configured".to_owned(),
            message: "boundaries/element-types found".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T37".to_owned(),
            severity: Severity::Error,
            title: "No import direction rules".to_owned(),
            message: "boundaries/element-types not found in ESLint config".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // T38: Entry-point barrel enforcement
    if content.contains("boundaries/entry-point") {
        results.push(CheckResult {
            id: "T38".to_owned(),
            severity: Severity::Info,
            title: "Entry-point enforcement configured".to_owned(),
            message: "boundaries/entry-point found".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T38".to_owned(),
            severity: Severity::Error,
            title: "No entry-point enforcement".to_owned(),
            message: "boundaries/entry-point not found in ESLint config".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // T39: External dependency per-zone bans
    if content.contains("boundaries/external") {
        results.push(CheckResult {
            id: "T39".to_owned(),
            severity: Severity::Info,
            title: "External dependency bans configured".to_owned(),
            message: "boundaries/external found".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T39".to_owned(),
            severity: Severity::Error,
            title: "No external dependency bans".to_owned(),
            message: "boundaries/external not found in ESLint config".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    results
}
