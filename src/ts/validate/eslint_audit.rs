use std::path::Path;

use crate::report::types::{CheckResult, Severity};

pub fn check(path: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    let eslint_path = path.join("eslint.config.mjs");
    if !eslint_path.exists() {
        // T1 already reports this — skip silently
        return results;
    }

    let content = match std::fs::read_to_string(&eslint_path) {
        Ok(c) => c,
        Err(_) => return results,
    };

    // T36: Zone definitions
    let has_zones = content.contains("element-types")
        || content.contains("domain")
            && (content.contains("commands") || content.contains("adapters"));

    if has_zones {
        results.push(CheckResult {
            id: "T36".to_string(),
            severity: Severity::Info,
            title: "Boundary zones configured".to_string(),
            message: "Zone definitions found in ESLint config".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T36".to_string(),
            severity: Severity::Error,
            title: "No boundary zones".to_string(),
            message: "No boundary zone definitions found in ESLint config".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    }

    // T37: Import direction rules
    if content.contains("boundaries/element-types") {
        results.push(CheckResult {
            id: "T37".to_string(),
            severity: Severity::Info,
            title: "Import direction rules configured".to_string(),
            message: "boundaries/element-types found".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T37".to_string(),
            severity: Severity::Error,
            title: "No import direction rules".to_string(),
            message: "boundaries/element-types not found in ESLint config".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    }

    // T38: Entry-point barrel enforcement
    if content.contains("boundaries/entry-point") {
        results.push(CheckResult {
            id: "T38".to_string(),
            severity: Severity::Info,
            title: "Entry-point enforcement configured".to_string(),
            message: "boundaries/entry-point found".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T38".to_string(),
            severity: Severity::Error,
            title: "No entry-point enforcement".to_string(),
            message: "boundaries/entry-point not found in ESLint config".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    }

    // T39: External dependency per-zone bans
    if content.contains("boundaries/external") {
        results.push(CheckResult {
            id: "T39".to_string(),
            severity: Severity::Info,
            title: "External dependency bans configured".to_string(),
            message: "boundaries/external found".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T39".to_string(),
            severity: Severity::Error,
            title: "No external dependency bans".to_string(),
            message: "boundaries/external not found in ESLint config".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    }

    results
}
