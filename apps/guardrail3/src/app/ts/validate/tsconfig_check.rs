use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

/// A tsconfig boolean check: (`setting_name`, `check_id`).
type TsConfigBool = (&'static str, &'static str);

/// A tsconfig string-value check: (`setting_name`, `expected_value`, `check_id`).
type TsConfigString = (&'static str, &'static str, &'static str);

/// Return a short explanation of what a tsconfig setting does and why it matters.
fn tsconfig_explanation(key: &str) -> &'static str {
    match key {
        "strict" => {
            " Enables all strict type-checking options (strictNullChecks, strictFunctionTypes, etc.), catching a wide class of bugs at compile time."
        }
        "noImplicitReturns" => {
            " Ensures every code path in a function returns a value, preventing undefined-at-runtime bugs."
        }
        "noUnusedLocals" => {
            " Catches declared but unused variables, which are dead code or indicate forgotten logic."
        }
        "noUnusedParameters" => {
            " Catches unused function parameters, which may indicate incomplete implementation."
        }
        "noUncheckedIndexedAccess" => {
            " Array/object index access returns `T | undefined` instead of just `T`, forcing null checks on dynamic access."
        }
        "exactOptionalPropertyTypes" => {
            " Distinguishes between `undefined` (property exists with no value) and missing (property not set), catching subtle bugs."
        }
        "forceConsistentCasingInFileNames" => {
            " Prevents import path casing mismatches that work on macOS/Windows but fail on Linux CI."
        }
        "isolatedModules" => {
            " Ensures each file can be independently transpiled (required by swc, esbuild, and other fast bundlers)."
        }
        "noPropertyAccessFromIndexSignature" => {
            " Forces bracket notation for index signature access, making it clear when a property might not exist."
        }
        "noImplicitOverride" => {
            " Requires explicit `override` keyword when overriding base class methods, catching accidental name collisions."
        }
        "noFallthroughCasesInSwitch" => {
            " Catches switch cases that fall through to the next case without `break`, which is almost always a bug."
        }
        "allowUnreachableCode" => {
            " When false, catches code after return/throw/break that can never execute, indicating logic errors."
        }
        "allowUnusedLabels" => {
            " When false, catches unused labels which are dead code and often indicate copy-paste errors."
        }
        "esModuleInterop" => {
            " Enables correct interop between CommonJS and ES modules, fixing default import issues with CJS packages."
        }
        _ => "",
    }
}

#[allow(clippy::too_many_lines, clippy::disallowed_methods)] // reason: comprehensive tsconfig validation; guardrail3 JSON config inspection
pub fn check_tsconfig(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let tsconfig_path = path.join("tsconfig.base.json");
    let tsconfig_path = if tsconfig_path.exists() {
        tsconfig_path
    } else {
        let alt = path.join("tsconfig.json");
        if alt.exists() {
            alt
        } else {
            results.push(CheckResult {
                id: "T9".to_owned(),
                severity: Severity::Error,
                title: "TypeScript config file not found".to_owned(),
                message: "No `tsconfig.base.json` or `tsconfig.json` found. The TypeScript compiler config \
                         controls type checking strictness, module resolution, and output settings. Without it, \
                         TypeScript uses permissive defaults that miss real bugs. Create `tsconfig.json` with \
                         `strict: true` and other guardrail settings, or run `guardrail3 ts generate`."
                    .to_owned(),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
            return;
        }
    };

    results.push(
        CheckResult {
            id: "T9".to_owned(),
            severity: Severity::Info,
            title: "TypeScript config exists".to_owned(),
            message: format!(
                "TypeScript compiler config found: `{}`.",
                tsconfig_path.display()
            ),
            file: Some(tsconfig_path.display().to_string()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );

    let Some(content) = fs.read_file(&tsconfig_path) else {
        return;
    };

    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "T9".to_owned(),
                severity: Severity::Error,
                title: "TypeScript config has invalid JSON".to_owned(),
                message: format!(
                    "Failed to parse tsconfig as JSON: {e}. The TypeScript compiler cannot read this file. \
                     Fix the JSON syntax error — common causes are trailing commas, missing quotes, or comments \
                     (use `jsonc` format if comments are needed)."
                ),
                file: Some(tsconfig_path.display().to_string()),
                line: None,
                inventory: false,
            });
            return;
        }
    };

    let compiler_options = json.get("compilerOptions");
    let required_bools: &[TsConfigBool] = &[
        ("strict", "T9"),
        ("noImplicitReturns", "T9"),
        ("noUnusedLocals", "T9"),
        ("noUnusedParameters", "T9"),
        ("noUncheckedIndexedAccess", "T52"),
        ("exactOptionalPropertyTypes", "T53"),
        ("forceConsistentCasingInFileNames", "T9"),
    ];

    // Settings that must be true
    let additional_required_bools: &[TsConfigBool] = &[
        ("noPropertyAccessFromIndexSignature", "T60"),
        ("noImplicitOverride", "T61"),
        ("noFallthroughCasesInSwitch", "T62"),
        ("isolatedModules", "T54"),
        ("esModuleInterop", "T68"),
    ];

    // Settings that must be false
    let required_false_bools: &[TsConfigBool] = &[
        ("allowUnreachableCode", "T63"),
        ("allowUnusedLabels", "T64"),
    ];

    let warn_bools: &[TsConfigBool] = &[];

    for (key, id) in required_bools {
        let val = compiler_options
            .and_then(|co| co.get(key))
            .and_then(serde_json::Value::as_bool);
        let explanation = tsconfig_explanation(key);

        match val {
            Some(true) => {
                results.push(
                    CheckResult {
                        id: (*id).to_owned(),
                        severity: Severity::Info,
                        title: format!("`{key}` enabled in tsconfig"),
                        message: format!("`{key}` is correctly set to `true`.{explanation}"),
                        file: Some(tsconfig_path.display().to_string()),
                        line: None,
                        inventory: false,
                    }
                    .as_inventory(),
                );
            }
            Some(false) => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("`{key}` disabled in tsconfig"),
                    message: format!(
                        "`{key}` is set to `false` but should be `true`.{explanation} \
                         Set `\"{key}\": true` in compilerOptions."
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
            None => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("`{key}` missing from tsconfig"),
                    message: format!(
                        "`{key}` not set in compilerOptions.{explanation} \
                         Add `\"{key}\": true` to compilerOptions in tsconfig."
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }

    for (key, id) in warn_bools {
        let val = compiler_options
            .and_then(|co| co.get(key))
            .and_then(serde_json::Value::as_bool);
        let explanation = tsconfig_explanation(key);

        match val {
            Some(true) => {
                results.push(
                    CheckResult {
                        id: (*id).to_owned(),
                        severity: Severity::Info,
                        title: format!("`{key}` enabled in tsconfig"),
                        message: format!("`{key}` is correctly set to `true`.{explanation}"),
                        file: Some(tsconfig_path.display().to_string()),
                        line: None,
                        inventory: false,
                    }
                    .as_inventory(),
                );
            }
            _ => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Warn,
                    title: format!("`{key}` not enabled in tsconfig"),
                    message: format!(
                        "`{key}` is not set to `true`.{explanation} \
                         Add `\"{key}\": true` to compilerOptions in tsconfig."
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }

    // Additional required true bools
    for (key, id) in additional_required_bools {
        let val = compiler_options
            .and_then(|co| co.get(key))
            .and_then(serde_json::Value::as_bool);
        let explanation = tsconfig_explanation(key);

        match val {
            Some(true) => {
                results.push(
                    CheckResult {
                        id: (*id).to_owned(),
                        severity: Severity::Info,
                        title: format!("`{key}` enabled in tsconfig"),
                        message: format!("`{key}` is correctly set to `true`.{explanation}"),
                        file: Some(tsconfig_path.display().to_string()),
                        line: None,
                        inventory: false,
                    }
                    .as_inventory(),
                );
            }
            Some(false) => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("`{key}` disabled in tsconfig"),
                    message: format!(
                        "`{key}` is set to `false` but should be `true`.{explanation} \
                         Set `\"{key}\": true` in compilerOptions."
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
            None => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("`{key}` missing from tsconfig"),
                    message: format!(
                        "`{key}` not set in compilerOptions.{explanation} \
                         Add `\"{key}\": true` to compilerOptions in tsconfig."
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }

    // Required false bools (must be explicitly set to false)
    for (key, id) in required_false_bools {
        let val = compiler_options
            .and_then(|co| co.get(key))
            .and_then(serde_json::Value::as_bool);
        let explanation = tsconfig_explanation(key);

        match val {
            Some(false) => {
                results.push(
                    CheckResult {
                        id: (*id).to_owned(),
                        severity: Severity::Info,
                        title: format!("`{key}` correctly set to false"),
                        message: format!("`{key}` is correctly set to `false`.{explanation}"),
                        file: Some(tsconfig_path.display().to_string()),
                        line: None,
                        inventory: false,
                    }
                    .as_inventory(),
                );
            }
            Some(true) => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("`{key}` incorrectly set to true"),
                    message: format!(
                        "`{key}` is set to `true` but should be `false`.{explanation} \
                         Set `\"{key}\": false` in compilerOptions."
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
            None => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("`{key}` missing from tsconfig"),
                    message: format!(
                        "`{key}` not set in compilerOptions.{explanation} \
                         Add `\"{key}\": false` to compilerOptions in tsconfig."
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }

    // T65-T67: Required string values for target, module, moduleResolution
    let required_strings: &[TsConfigString] = &[
        ("target", "es2022", "T65"),
        ("module", "esnext", "T66"),
        ("moduleResolution", "bundler", "T67"),
    ];

    for &(key, expected, id) in required_strings {
        let val = compiler_options
            .and_then(|co| co.get(key))
            .and_then(serde_json::Value::as_str);

        match val {
            Some(actual) if actual.eq_ignore_ascii_case(expected) => {
                results.push(
                    CheckResult {
                        id: id.to_owned(),
                        severity: Severity::Info,
                        title: format!("`{key}` correctly set in tsconfig"),
                        message: format!("`{key}` is correctly set to `\"{actual}\"`."),
                        file: Some(tsconfig_path.display().to_string()),
                        line: None,
                        inventory: false,
                    }
                    .as_inventory(),
                );
            }
            Some(actual) => {
                results.push(CheckResult {
                    id: id.to_owned(),
                    severity: Severity::Error,
                    title: format!("`{key}` has wrong value in tsconfig"),
                    message: format!(
                        "`{key}` is set to `\"{actual}\"` but should be `\"{expected}\"`. \
                         Set `\"{key}\": \"{expected}\"` in compilerOptions."
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
            None => {
                results.push(CheckResult {
                    id: id.to_owned(),
                    severity: Severity::Error,
                    title: format!("`{key}` missing from tsconfig"),
                    message: format!(
                        "`{key}` not set in compilerOptions. \
                         Add `\"{key}\": \"{expected}\"` to compilerOptions in tsconfig."
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }

    // T10: Extra compiler options — Info inventory
    let known_keys: &[&str] = &[
        "strict",
        "noImplicitReturns",
        "noUnusedLocals",
        "noUnusedParameters",
        "noUncheckedIndexedAccess",
        "exactOptionalPropertyTypes",
        "forceConsistentCasingInFileNames",
        "isolatedModules",
        "noPropertyAccessFromIndexSignature",
        "noImplicitOverride",
        "noFallthroughCasesInSwitch",
        "allowUnreachableCode",
        "allowUnusedLabels",
        // Common standard options (not flagged as "extra")
        "target",
        "module",
        "moduleResolution",
        "lib",
        "jsx",
        "outDir",
        "rootDir",
        "baseUrl",
        "paths",
        "declaration",
        "declarationMap",
        "sourceMap",
        "esModuleInterop",
        "allowImportingTsExtensions",
        "noEmit",
        "resolveJsonModule",
        "skipLibCheck",
        "incremental",
        "tsBuildInfoFile",
        "allowJs",
        "plugins",
        "customConditions",
        "verbatimModuleSyntax",
    ];

    if let Some(co) = compiler_options.and_then(|co| co.as_object()) {
        for key in co.keys() {
            if !known_keys.contains(&key.as_str()) {
                results.push(CheckResult {
                    id: "T10".to_owned(),
                    severity: Severity::Info,
                    title: format!("Extra tsconfig compilerOption: `{key}`"),
                    message: format!(
                        "Non-standard compilerOption `{key}` = {}. This setting is not in the guardrail baseline. \
                         Verify it is intentional and document why it's needed.",
                        co.get(key)
                            .map_or_else(|| "?".to_owned(), std::string::ToString::to_string)
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                    inventory: false,
                }.as_inventory());
            }
        }
    }
}
