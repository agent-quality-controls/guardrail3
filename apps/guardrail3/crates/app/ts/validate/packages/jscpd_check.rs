use std::path::{Path, PathBuf};

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

#[allow(clippy::too_many_lines, clippy::disallowed_methods)] // reason: jscpd config validation; guardrail3 JSON config inspection — not a trust boundary
pub fn check_jscpd(
    fs: &dyn FileSystem,
    jscpd_configs: &[PathBuf],
    root: &Path,
    results: &mut Vec<CheckResult>,
) {
    if jscpd_configs.is_empty() {
        results.push(CheckResult::from_parts(
    "T19".to_owned(),
    Severity::Warn,
    "Copy-paste detection config `.jscpd.json` not found".to_owned(),
    "No `.jscpd.json` found at project root. jscpd detects copy-pasted code blocks that should \
                     be extracted into shared functions. Without config, jscpd uses defaults that may miss \
                     duplicates or produce false positives. Create `.jscpd.json` with `threshold: 0` and \
                     appropriate ignore patterns, or run `guardrail3 ts generate`."
                .to_owned(),
    Some(root.display().to_string()),
    None,
    false,
        ));
        return;
    }

    for jscpd_path in jscpd_configs {
        results.push(
            CheckResult::from_parts(
                "T19".to_owned(),
                Severity::Info,
                "Copy-paste detection config `.jscpd.json` exists".to_owned(),
                format!(
                    "jscpd configuration file found: `{}`.",
                    jscpd_path.display()
                ),
                Some(jscpd_path.display().to_string()),
                None,
                false,
            )
            .as_inventory(),
        );

        let Some(content) = fs.read_file(jscpd_path) else {
            continue;
        };

        let content = content.strip_prefix('\u{FEFF}').unwrap_or(&content);

        let json: serde_json::Value = match serde_json::from_str(content) {
            Ok(v) => v,
            Err(e) => {
                results.push(CheckResult::from_parts(
    "T19".to_owned(),
    Severity::Error,
    "`.jscpd.json` has invalid JSON".to_owned(),
    format!(
                        "Failed to parse `.jscpd.json`: {e}. Fix the JSON syntax error so jscpd can read its config."
                    ),
    Some(jscpd_path.display().to_string()),
    None,
    false,
                });
                continue;
            }
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
                    results.push(
                    CheckResult::from_parts(
                        "T22".to_owned(),
                        Severity::Info,
                        "jscpd ignore pattern configured".to_owned(),
                        format!(
                            "Ignore pattern: `{p}`. Files matching this pattern are excluded from \
                         copy-paste detection. Verify this exclusion is justified."
                        ),
                        Some(jscpd_path.display().to_string()),
                        None,
                        false,
                    )
                    .as_inventory(),
                );
                }
            }
        }

        // T-JSCPD-01: minTokens field missing
        if json.get("minTokens").is_none() {
            results.push(CheckResult {
            id: "T-JSCPD-01".to_owned(),
            severity: Severity::Warn,
            title: "jscpd `minTokens` field missing".to_owned(),
            message: "No `minTokens` field in `.jscpd.json`. Without this, jscpd uses its default minimum \
                     token count for duplicate detection, which may not be appropriate for the project. \
                     Set `\"minTokens\": 50` explicitly (or adjust to the desired threshold)."
                .to_owned(),
            file: Some(jscpd_path.display().to_string()),
            line: None,
            inventory: false,
        });
        }

        // T-JSCPD-02: absolute field missing or not true
        match json.get("absolute") {
            Some(serde_json::Value::Bool(true)) => {}
            _ => {
                results.push(CheckResult {
                id: "T-JSCPD-02".to_owned(),
                severity: Severity::Warn,
                title: "jscpd config missing `absolute: true`".to_owned(),
                message: "jscpd config missing `absolute: true` — needed for meaningful paths in monorepo output"
                    .to_owned(),
                file: Some(jscpd_path.display().to_string()),
                line: None,
                inventory: false,
            });
            }
        }

        // T-JSCPD-03: Required ignore patterns
        let required_patterns: &[&str] = &[
            "**/node_modules/**",
            "**/.next/**",
            "**/dist/**",
            "**/target/**",
            "**/components/ui/**",
        ];

        let configured_ignores: Vec<&str> = json
            .get("ignore")
            .and_then(serde_json::Value::as_array)
            .map(|arr| arr.iter().filter_map(serde_json::Value::as_str).collect())
            .unwrap_or_default();

        for required in required_patterns {
            if !configured_ignores.iter().any(|p| p == required) {
                results.push(CheckResult {
                id: "T-JSCPD-03".to_owned(),
                severity: Severity::Warn,
                title: "jscpd missing required ignore pattern".to_owned(),
                message: format!(
                    "Required ignore pattern `{required}` not found in `.jscpd.json` `ignore` array. \
                     Add it to prevent false-positive duplication reports from generated or vendored files."
                ),
                file: Some(jscpd_path.display().to_string()),
                line: None,
                inventory: false,
            });
            }
        }

        // T-JSCPD-04: format field missing
        if json.get("format").is_none() {
            results.push(CheckResult {
                id: "T-JSCPD-04".to_owned(),
                severity: Severity::Warn,
                title: "jscpd `format` field missing".to_owned(),
                message:
                    "jscpd config should specify `format` to explicitly list scanned languages \
                     (e.g., `[\"typescript\"]`)."
                        .to_owned(),
                file: Some(jscpd_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    } // end for jscpd_path,
)

// T60: Content import restriction
pub fn check_content_import_restriction(
    fs: &dyn FileSystem,
    eslint_configs: &[PathBuf],
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
    // Only applies if there's a landing/content app
    let landing_dir = path.join("apps").join("landing");
    if !landing_dir.exists() {
        return;
    }

    // Use first eslint config found
    let Some(eslint_path) = eslint_configs.first() else {
        return;
    };

    let Some(content) = fs.read_file(eslint_path) else {
        return;
    };

    if content.contains("content/") || content.contains("content/**") {
        results.push(CheckResult::from_parts(
    "T60".to_owned(),
    Severity::Info,
    "Content directory import restriction configured".to_owned(),
    "Content import restriction pattern found in ESLint config. This prevents application \
                     code from importing raw content files directly, enforcing access through the content API."
                .to_owned(),
    Some(eslint_path.display().to_string()),
    None,
    false,
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
        ));
    }

// T61: Velite config exists
pub fn check_velite_config(velite_configs: &[PathBuf], results: &mut Vec<CheckResult>) {
    for velite_path in velite_configs {
        results.push(
            CheckResult::from_parts(
                "T61".to_owned(),
                Severity::Info,
                "Velite content config exists".to_owned(),
                format!(
                    "Velite config found: `{}`. Velite processes MDX/markdown content into typed, \
                 validated collections at build time.",
                    velite_path.display()
                ),
                Some(velite_path.display().to_string()),
                None,
                false,
            )
            .as_inventory(),
        );
    },
)
