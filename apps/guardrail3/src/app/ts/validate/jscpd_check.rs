use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

#[allow(clippy::too_many_lines, clippy::disallowed_methods)] // reason: jscpd config validation; guardrail3 JSON config inspection — not a trust boundary
pub fn check_jscpd(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let jscpd_path = path.join(".jscpd.json");
    if !jscpd_path.exists() {
        results.push(CheckResult {
            id: "T19".to_owned(),
            severity: Severity::Warn,
            title: ".jscpd.json missing".to_owned(),
            message: "No .jscpd.json found at project root".to_owned(),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    results.push(CheckResult {
        id: "T19".to_owned(),
        severity: Severity::Info,
        title: ".jscpd.json exists".to_owned(),
        message: "Found at project root".to_owned(),
        file: Some(jscpd_path.display().to_string()),
        line: None,
        inventory: false,
    }.as_inventory());

    let Some(content) = fs.read_file(&jscpd_path) else {
        return;
    };

    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return,
    };

    // T20: threshold = 0
    match json.get("threshold") {
        Some(serde_json::Value::Number(n)) => {
            let val = n.as_f64().unwrap_or(1.0);
            if val == 0.0 {
                results.push(CheckResult {
                    id: "T20".to_owned(),
                    severity: Severity::Info,
                    title: "jscpd threshold correct".to_owned(),
                    message: "threshold = 0".to_owned(),
                    file: Some(jscpd_path.display().to_string()),
                    line: None,
                    inventory: false,
                }.as_inventory());
            } else {
                results.push(CheckResult {
                    id: "T20".to_owned(),
                    severity: Severity::Error,
                    title: "jscpd threshold not 0".to_owned(),
                    message: format!("threshold = {n}, expected 0"),
                    file: Some(jscpd_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
        }
        _ => {
            results.push(CheckResult {
                id: "T20".to_owned(),
                severity: Severity::Error,
                title: "jscpd threshold missing".to_owned(),
                message: "No threshold field in .jscpd.json".to_owned(),
                file: Some(jscpd_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    // T21: minTokens differs from 50
    if let Some(serde_json::Value::Number(n)) = json.get("minTokens") {
        let val = n.as_u64().unwrap_or(0);
        if val != 50 {
            results.push(CheckResult {
                id: "T21".to_owned(),
                severity: Severity::Info,
                title: "jscpd minTokens non-default".to_owned(),
                message: format!("minTokens = {val} (default is 50)"),
                file: Some(jscpd_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        }
    }

    // T22: Extra ignore patterns
    if let Some(serde_json::Value::Array(ignore)) = json.get("ignore") {
        for pattern in ignore {
            if let Some(p) = pattern.as_str() {
                results.push(CheckResult {
                    id: "T22".to_owned(),
                    severity: Severity::Info,
                    title: "jscpd ignore pattern".to_owned(),
                    message: p.to_owned(),
                    file: Some(jscpd_path.display().to_string()),
                    line: None,
                    inventory: false,
                }.as_inventory());
            }
        }
    }
}

// T60: Content import restriction
pub fn check_content_import_restriction(
    fs: &dyn FileSystem,
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
    // Only applies if there's a landing/content app
    let landing_dir = path.join("apps").join("landing");
    if !landing_dir.exists() {
        return;
    }

    let eslint_path = path.join("eslint.config.mjs");
    if !eslint_path.exists() {
        return;
    }

    let Some(content) = fs.read_file(&eslint_path) else {
        return;
    };

    if content.contains("content/") || content.contains("content/**") {
        results.push(CheckResult {
            id: "T60".to_owned(),
            severity: Severity::Info,
            title: "Content import restriction configured".to_owned(),
            message: "Content import restriction pattern found in ESLint config".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T60".to_owned(),
            severity: Severity::Warn,
            title: "No content import restriction".to_owned(),
            message: "No content/ import restriction in ESLint config (landing app detected)"
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

// T61: Velite config exists
pub fn check_velite_config(path: &Path, results: &mut Vec<CheckResult>) {
    let landing_dir = path.join("apps").join("landing");
    if !landing_dir.exists() {
        return;
    }

    let velite_path = landing_dir.join("velite.config.mjs");
    let velite_ts_path = landing_dir.join("velite.config.ts");

    if velite_path.exists() || velite_ts_path.exists() {
        let found_path = if velite_path.exists() {
            &velite_path
        } else {
            &velite_ts_path
        };
        results.push(CheckResult {
            id: "T61".to_owned(),
            severity: Severity::Info,
            title: "Velite config exists".to_owned(),
            message: format!("Found: {}", found_path.display()),
            file: Some(found_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    }
}
