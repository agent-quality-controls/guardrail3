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
            title: "Copy-paste detection config `.jscpd.json` not found".to_owned(),
            message: "No `.jscpd.json` found at project root. jscpd detects copy-pasted code blocks that should \
                     be extracted into shared functions. Without config, jscpd uses defaults that may miss \
                     duplicates or produce false positives. Create `.jscpd.json` with `threshold: 0` and \
                     appropriate ignore patterns, or run `guardrail3 ts generate`."
                .to_owned(),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    results.push(CheckResult {
        id: "T19".to_owned(),
        severity: Severity::Info,
        title: "Copy-paste detection config `.jscpd.json` exists".to_owned(),
        message: "jscpd configuration file found at project root.".to_owned(),
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
                    title: "jscpd threshold correctly set to 0".to_owned(),
                    message: "`threshold` = 0. Zero tolerance for copy-paste duplication — any detected \
                             duplicate block is reported."
                        .to_owned(),
                    file: Some(jscpd_path.display().to_string()),
                    line: None,
                    inventory: false,
                }.as_inventory());
            } else {
                results.push(CheckResult {
                    id: "T20".to_owned(),
                    severity: Severity::Error,
                    title: "jscpd threshold is not 0".to_owned(),
                    message: format!(
                        "`threshold` = {n}, expected 0. A non-zero threshold allows a percentage of \
                         duplication before reporting, hiding copy-paste problems. Set `\"threshold\": 0` \
                         in `.jscpd.json` for zero tolerance."
                    ),
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
                title: "jscpd `threshold` field missing".to_owned(),
                message: "No `threshold` field in `.jscpd.json`. Without this, jscpd uses a default threshold \
                         that allows some duplication. Set `\"threshold\": 0` for zero tolerance."
                    .to_owned(),
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
                title: "jscpd `minTokens` set to non-default value".to_owned(),
                message: format!(
                    "`minTokens` = {val} (default is 50). This controls the minimum duplicate block size — \
                     lower values catch smaller duplicates but may produce false positives. \
                     Verify this value is appropriate for the project."
                ),
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
                    title: "jscpd ignore pattern configured".to_owned(),
                    message: format!(
                        "Ignore pattern: `{p}`. Files matching this pattern are excluded from \
                         copy-paste detection. Verify this exclusion is justified."
                    ),
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
            title: "Content directory import restriction configured".to_owned(),
            message: "Content import restriction pattern found in ESLint config. This prevents application \
                     code from importing raw content files directly, enforcing access through the content API."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T60".to_owned(),
            severity: Severity::Warn,
            title: "No content directory import restriction".to_owned(),
            message: "Landing app detected but no `content/` import restriction in ESLint config. Without this, \
                     components can import raw content files directly instead of going through the content API, \
                     bypassing processing and validation. Add a `no-restricted-imports` rule for `content/` paths."
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
            title: "Velite content config exists".to_owned(),
            message: format!(
                "Velite config found: `{}`. Velite processes MDX/markdown content into typed, \
                 validated collections at build time.",
                found_path.display()
            ),
            file: Some(found_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    }
}
