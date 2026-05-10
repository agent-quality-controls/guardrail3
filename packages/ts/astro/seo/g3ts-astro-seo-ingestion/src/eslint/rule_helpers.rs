use std::collections::BTreeSet;

/// `SOURCE_MODULE_EXTENSIONS` constant.
const SOURCE_MODULE_EXTENSIONS: [&str; 9] = [
    ".ts", ".tsx", ".js", ".jsx", ".mts", ".cts", ".mjs", ".cjs", ".astro",
];

/// Alias for the JSON object representation of an `ESLint` rule's option object.
type JsonOptionObject = serde_json::Map<String, serde_json::Value>;

/// `rule_setting_has_expected_module_globs`: rule setting has expected module globs.
pub(crate) fn rule_setting_has_expected_module_globs(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    key: &str,
    expected_sources: &[String],
) -> bool {
    let expected = expected_module_globs(expected_sources);
    !expected.is_empty()
        && string_arrays_match_as_sets(&string_array_option(setting, key), &expected)
}

/// `expected_module_globs`: expected module globs.
fn expected_module_globs(source_paths: &[String]) -> Vec<String> {
    let mut globs = source_paths
        .iter()
        .map(|source_path| {
            let source_path = source_path.trim_end_matches('/');
            if is_source_module_file(source_path) {
                normalize_glob(source_path)
            } else {
                format!("{}/**/*", normalize_glob(source_path))
            }
        })
        .collect::<Vec<_>>();
    globs.sort();
    globs.dedup();
    globs
}

/// `string_arrays_match_as_sets`: string arrays match as sets.
fn string_arrays_match_as_sets(left: &[String], right: &[String]) -> bool {
    let left_set: BTreeSet<String> = left.iter().map(|value| normalize_glob(value)).collect();
    let right_set: BTreeSet<String> = right.iter().map(|value| normalize_glob(value)).collect();
    left_set == right_set
}

/// `rule_setting_option_globs_match_any_path`: rule setting option globs match any path.
pub(crate) fn rule_setting_option_globs_match_any_path(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
    candidate_paths: &[String],
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        non_empty_string_array_option(object.get(option_name))
            .is_some_and(|patterns| all_paths_match_globs(&patterns, candidate_paths))
    })
}

/// `rule_setting_option_globs_are_valid`: rule setting option globs are valid.
pub(crate) fn rule_setting_option_globs_are_valid(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        non_empty_string_array_option(object.get(option_name))
            .is_some_and(|patterns| globs_are_valid(&patterns))
    })
}

/// `first_option_object`: first option object.
fn first_option_object(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> Option<&JsonOptionObject> {
    setting
        .options
        .first()
        .and_then(serde_json::Value::as_object)
}

/// `rule_setting_is_error`: rule setting is error.
pub(crate) fn rule_setting_is_error(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
}

/// `probe_has_pipeline_plugin_package`: probe has pipeline plugin package.
pub(crate) fn probe_has_pipeline_plugin_package(
    probe: &eslint_config_parser::types::EslintEffectiveConfigProbe,
) -> bool {
    probe
        .plugin_package_names
        .get("astro-pipeline")
        .is_some_and(|package_names| {
            package_names
                .iter()
                .any(|name| name == "g3ts-eslint-plugin-astro-pipeline")
        })
}

/// `non_empty_string_array_option`: non empty string array option.
fn non_empty_string_array_option(option: Option<&serde_json::Value>) -> Option<Vec<&str>> {
    let values = option.and_then(serde_json::Value::as_array)?;

    if values.is_empty() {
        return None;
    }

    let mut strings = Vec::with_capacity(values.len());

    for value in values {
        let text = value.as_str()?.trim();
        if text.is_empty() {
            return None;
        }
        strings.push(text);
    }

    Some(strings)
}

/// `string_array_option`: string array option.
fn string_array_option(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
) -> Vec<String> {
    first_option_object(setting)
        .and_then(|object| object.get(option_name))
        .and_then(serde_json::Value::as_array)
        .map_or_else(Vec::new, |values| {
            values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .collect()
        })
}

/// `all_paths_match_globs`: all paths match globs.
fn all_paths_match_globs(patterns: &[&str], candidate_paths: &[String]) -> bool {
    let mut builder = globset::GlobSetBuilder::new();

    for pattern in patterns {
        let Ok(glob) = globset::Glob::new(&normalize_glob(pattern)) else {
            return false;
        };
        let _ = builder.add(glob);
    }

    let Ok(glob_set) = builder.build() else {
        return false;
    };

    candidate_paths
        .iter()
        .all(|candidate_path| glob_set.is_match(normalize_glob(candidate_path)))
}

/// `globs_are_valid`: globs are valid.
fn globs_are_valid(patterns: &[&str]) -> bool {
    let mut builder = globset::GlobSetBuilder::new();

    for pattern in patterns {
        let Ok(glob) = globset::Glob::new(&normalize_glob(pattern)) else {
            return false;
        };
        let _ = builder.add(glob);
    }

    builder.build().is_ok()
}

/// `normalize_glob`: normalize glob.
fn normalize_glob(value: &str) -> String {
    let mut normalized = value.replace('\\', "/");
    while normalized.contains("//") {
        normalized = normalized.replace("//", "/");
    }
    normalized.trim_start_matches("./").to_owned()
}

/// `is_source_module_file`: is source module file.
fn is_source_module_file(rel_path: &str) -> bool {
    SOURCE_MODULE_EXTENSIONS
        .iter()
        .any(|extension| rel_path.ends_with(extension))
}
