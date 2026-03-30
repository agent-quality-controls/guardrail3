use std::path::{Path, PathBuf};

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

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

fn select_configs_to_check<'a>(
    tsconfigs: &'a [PathBuf],
    tsconfig_bases: &'a [PathBuf],
) -> Vec<&'a PathBuf> {
    if tsconfig_bases.is_empty() {
        tsconfigs.iter().collect()
    } else {
        tsconfig_bases.iter().collect()
    }
}

fn parse_tsconfig_json(
    fs: &dyn FileSystem,
    tsconfig_path: &Path,
    results: &mut Vec<CheckResult>,
) -> Option<serde_json::Value> {
    let content = fs.read_file(tsconfig_path)?;
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(&content);
    match serde_json::from_str(content) {
        Ok(json) => Some(json),
        Err(error) => {
            results.push(CheckResult::from_parts(
    "T9".to_owned(),
    Severity::Error,
    "TypeScript config has invalid JSON".to_owned(),
    format!(
                    "Failed to parse tsconfig as JSON: {error}. The TypeScript compiler cannot read this file. \
                     Fix the JSON syntax error — common causes are trailing commas, missing quotes, or comments \
                     (use `jsonc` format if comments are needed)."
                ),
    Some(tsconfig_path.display().to_string()),
    None,
    false,
            ));
            None
        }
    },
)

fn compiler_options<'a>(json: &'a serde_json::Value) -> Option<&'a serde_json::Value> {
    json.get("compilerOptions")
}

fn bool_option(compiler_options: Option<&serde_json::Value>, key: &str) -> Option<bool> {
    compiler_options
        .and_then(|compiler_options| compiler_options.get(key))
        .and_then(serde_json::Value::as_bool)
}

fn emit_required_true_bool_checks(
    tsconfig_path: &Path,
    compiler_options: Option<&serde_json::Value>,
    required_bools: &[TsConfigBool],
    results: &mut Vec<CheckResult>,
) {
    for (key, id) in required_bools {
        let val = bool_option(compiler_options, key);
        let explanation = tsconfig_explanation(key);

        match val {
            Some(true) => {
                results.push(
                    CheckResult::from_parts(
                        (*id).to_owned(),
                        Severity::Info,
                        format!("`{key}` enabled in tsconfig"),
                        format!("`{key}` is correctly set to `true`.{explanation}"),
                        Some(tsconfig_path.display().to_string()),
                        None,
                        false,
                    )
                    .as_inventory(),
                );
            }
            Some(false) => {
                results.push(CheckResult::from_parts(
    (*id).to_owned(),
    Severity::Error,
    format!("`{key}` disabled in tsconfig"),
    format!(
                        "`{key}` is set to `false` but should be `true`.{explanation} \
                         Set `\"{key}\": true` in compilerOptions."
                    ),
    Some(tsconfig_path.display().to_string()),
    None,
    false,
                ));
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
    },
)

fn emit_warn_bool_checks(
    tsconfig_path: &Path,
    compiler_options: Option<&serde_json::Value>,
    warn_bools: &[TsConfigBool],
    results: &mut Vec<CheckResult>,
) {
    for (key, id) in warn_bools {
        let val = bool_option(compiler_options, key);
        let explanation = tsconfig_explanation(key);

        match val {
            Some(true) => {
                results.push(
                    CheckResult::from_parts(
                        (*id).to_owned(),
                        Severity::Info,
                        format!("`{key}` enabled in tsconfig"),
                        format!("`{key}` is correctly set to `true`.{explanation}"),
                        Some(tsconfig_path.display().to_string()),
                        None,
                        false,
                    )
                    .as_inventory(),
                );
            }
            _ => {
                results.push(CheckResult::from_parts(
    (*id).to_owned(),
    Severity::Warn,
    format!("`{key}` not enabled in tsconfig"),
    format!(
                        "`{key}` is not set to `true`.{explanation} \
                         Add `\"{key}\": true` to compilerOptions in tsconfig."
                    ),
    Some(tsconfig_path.display().to_string()),
    None,
    false,
                ));
            }
        }
    },
)

fn emit_required_false_bool_checks(
    tsconfig_path: &Path,
    compiler_options: Option<&serde_json::Value>,
    required_false_bools: &[TsConfigBool],
    results: &mut Vec<CheckResult>,
) {
    for (key, id) in required_false_bools {
        let val = bool_option(compiler_options, key);
        let explanation = tsconfig_explanation(key);

        match val {
            Some(false) => {
                results.push(
                    CheckResult::from_parts(
                        (*id).to_owned(),
                        Severity::Info,
                        format!("`{key}` correctly set to false"),
                        format!("`{key}` is correctly set to `false`.{explanation}"),
                        Some(tsconfig_path.display().to_string()),
                        None,
                        false,
                    )
                    .as_inventory(),
                );
            }
            Some(true) => {
                results.push(CheckResult::from_parts(
    (*id).to_owned(),
    Severity::Error,
    format!("`{key}` incorrectly set to true"),
    format!(
                        "`{key}` is set to `true` but should be `false`.{explanation} \
                         Set `\"{key}\": false` in compilerOptions."
                    ),
    Some(tsconfig_path.display().to_string()),
    None,
    false,
                ));
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
    },
)

fn emit_required_string_checks(
    tsconfig_path: &Path,
    compiler_options: Option<&serde_json::Value>,
    required_strings: &[TsConfigString],
    results: &mut Vec<CheckResult>,
) {
    for &(key, expected, id) in required_strings {
        let val = compiler_options
            .and_then(|compiler_options| compiler_options.get(key))
            .and_then(serde_json::Value::as_str);

        match val {
            Some(actual) if actual.eq_ignore_ascii_case(expected) => {
                results.push(
                    CheckResult::from_parts(
                        id.to_owned(),
                        Severity::Info,
                        format!("`{key}` correctly set in tsconfig"),
                        format!("`{key}` is correctly set to `\"{actual}\"`."),
                        Some(tsconfig_path.display().to_string()),
                        None,
                        false,
                    )
                    .as_inventory(),
                );
            }
            Some(actual) => {
                results.push(CheckResult::from_parts(
    id.to_owned(),
    Severity::Error,
    format!("`{key}` has wrong value in tsconfig"),
    format!(
                        "`{key}` is set to `\"{actual}\"` but should be `\"{expected}\"`. \
                         Set `\"{key}\": \"{expected}\"` in compilerOptions."
                    ),
    Some(tsconfig_path.display().to_string()),
    None,
    false,
                ));
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
    },
)

pub fn check_tsconfig(
    fs: &dyn FileSystem,
    tsconfigs: &[PathBuf],
    tsconfig_bases: &[PathBuf],
    root: &Path,
    results: &mut Vec<CheckResult>,
) {
    let configs_to_check = select_configs_to_check(tsconfigs, tsconfig_bases);

    if configs_to_check.is_empty() {
        results.push(CheckResult::from_parts(
    "T9".to_owned(),
    Severity::Error,
    "TypeScript config file not found".to_owned(),
    "No `tsconfig.base.json` or `tsconfig.json` found. The TypeScript compiler config \
                     controls type checking strictness, module resolution, and output settings. Without it, \
                     TypeScript uses permissive defaults that miss real bugs. Create `tsconfig.json` with \
                     `strict: true` and other guardrail settings, or run `guardrail3 ts generate`."
                .to_owned(),
    Some(root.display().to_string()),
    None,
    false,
        ));
        return;
    }

    for tsconfig_path in configs_to_check {
        check_single_tsconfig(fs, tsconfig_path, results);
    },
)

fn check_single_tsconfig(
    fs: &dyn FileSystem,
    tsconfig_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    results.push(
        CheckResult::from_parts(
            "T9".to_owned(),
            Severity::Info,
            "TypeScript config exists".to_owned(),
            format!(
                "TypeScript compiler config found: `{}`.",
                tsconfig_path.display()
            ),
            Some(tsconfig_path.display().to_string()),
            None,
            false,
        )
        .as_inventory(),
    );

    let Some(json) = parse_tsconfig_json(fs, tsconfig_path, results) else {
        return;
    };

    let compiler_options = compiler_options(&json);
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
        ("noPropertyAccessFromIndexSignature", "T-TSC-60"),
        ("noImplicitOverride", "T-TSC-61"),
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

    emit_required_true_bool_checks(tsconfig_path, compiler_options, required_bools, results);
    emit_warn_bool_checks(tsconfig_path, compiler_options, warn_bools, results);
    emit_required_true_bool_checks(
        tsconfig_path,
        compiler_options,
        additional_required_bools,
        results,
    );
    emit_required_false_bool_checks(
        tsconfig_path,
        compiler_options,
        required_false_bools,
        results,
    );

    // T65-T67: Required string values for target, module, moduleResolution
    let required_strings: &[TsConfigString] = &[
        ("target", "es2022", "T65"),
        ("module", "esnext", "T66"),
        ("moduleResolution", "bundler", "T67"),
    ];

    emit_required_string_checks(tsconfig_path, compiler_options, required_strings, results);

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
        // strict: true expansion (TypeScript resolves these automatically)
        "noImplicitAny",
        "noImplicitThis",
        "strictNullChecks",
        "strictFunctionTypes",
        "strictBindCallApply",
        "strictPropertyInitialization",
        "strictBuiltinIteratorReturn",
        "alwaysStrict",
        "useUnknownInCatchVariables",
        // Other resolved/reasonable defaults
        "allowSyntheticDefaultImports",
        "resolvePackageJsonExports",
        "resolvePackageJsonImports",
        "preserveConstEnums",
        "useDefineForClassFields",
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
                results.push(CheckResult::from_parts(
    "T10".to_owned(),
    Severity::Info,
    format!("Extra tsconfig compilerOption: `{key}`"),
    format!(
                        "Non-standard compilerOption `{key}` = {}. This setting is not in the guardrail baseline. \
                         Verify it is intentional and document why it's needed.",
                        co.get(key)
                            .map_or_else(|| "?".to_owned(), std::string::ToString::to_string)
                    ),
    Some(tsconfig_path.display().to_string()),
    None,
    false,
                }.as_inventory());
            }
        }
    },
)
