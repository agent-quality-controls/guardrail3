//! Helper functions for `ESLint` rule and option introspection.

use std::collections::BTreeSet;

/// File extensions treated as source modules for glob expansion.
const SOURCE_MODULE_EXTENSIONS: [&str; 9] = [
    ".ts", ".tsx", ".js", ".jsx", ".mts", ".cts", ".mjs", ".cjs", ".astro",
];

/// `rule_setting_has_expected_module_globs` helper.
pub(super) fn rule_setting_has_expected_module_globs(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    key: &str,
    expected_sources: &[String],
) -> bool {
    let expected = expected_module_globs(expected_sources);
    !expected.is_empty()
        && string_arrays_match_as_sets(&string_array_option(setting, key), &expected)
}

/// `expected_module_globs` helper.
pub(super) fn expected_module_globs(source_paths: &[String]) -> Vec<String> {
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

/// `string_arrays_match_as_sets` helper.
pub(super) fn string_arrays_match_as_sets(left: &[String], right: &[String]) -> bool {
    let left_set: BTreeSet<String> = left.iter().map(|value| normalize_glob(value)).collect();
    let right_set: BTreeSet<String> = right.iter().map(|value| normalize_glob(value)).collect();
    left_set == right_set
}

/// `rule_setting_has_option_globs_coverage` helper.
pub(super) fn rule_setting_has_option_globs_coverage(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    key: &str,
    candidate_paths: &[String],
) -> bool {
    if candidate_paths.is_empty() {
        return rule_setting_option_globs_are_valid(setting, key);
    }

    rule_setting_option_globs_match_any_path(setting, key, candidate_paths)
}

/// `rule_setting_option_globs_match_any_path` helper.
pub(super) fn rule_setting_option_globs_match_any_path(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
    candidate_paths: &[String],
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        non_empty_string_array_option(object.get(option_name))
            .is_some_and(|patterns| all_paths_match_globs(&patterns, candidate_paths))
    })
}

/// `rule_setting_option_globs_are_valid` helper.
pub(super) fn rule_setting_option_globs_are_valid(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        non_empty_string_array_option(object.get(option_name))
            .is_some_and(|patterns| globs_are_valid(&patterns))
    })
}

/// JSON object map type used for `ESLint` rule option payloads.
pub(super) type EslintRuleOptionMap = serde_json::Map<String, serde_json::Value>;

/// `first_option_object` helper.
pub(super) fn first_option_object(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> Option<&EslintRuleOptionMap> {
    setting
        .options
        .first()
        .and_then(serde_json::Value::as_object)
}

/// `rule_setting_is_error` helper.
pub(super) fn rule_setting_is_error(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
}

/// `probe_has_pipeline_plugin_package` helper.
pub(super) fn probe_has_pipeline_plugin_package(
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

/// `non_empty_string_array_option` helper.
pub(super) fn non_empty_string_array_option(
    option: Option<&serde_json::Value>,
) -> Option<Vec<&str>> {
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

/// `string_array_option` helper.
pub(super) fn string_array_option(
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

/// `all_paths_match_globs` helper.
pub(super) fn all_paths_match_globs(patterns: &[&str], candidate_paths: &[String]) -> bool {
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

/// `globs_are_valid` helper.
pub(super) fn globs_are_valid(patterns: &[&str]) -> bool {
    let mut builder = globset::GlobSetBuilder::new();

    for pattern in patterns {
        let Ok(glob) = globset::Glob::new(&normalize_glob(pattern)) else {
            return false;
        };
        let _ = builder.add(glob);
    }

    builder.build().is_ok()
}

/// `normalize_glob` helper.
pub(super) fn normalize_glob(value: &str) -> String {
    let mut normalized = value.replace('\\', "/");
    while normalized.contains("//") {
        normalized = normalized.replace("//", "/");
    }
    normalized.trim_start_matches("./").to_owned()
}

/// `is_source_module_file` helper.
fn is_source_module_file(rel_path: &str) -> bool {
    SOURCE_MODULE_EXTENSIONS
        .iter()
        .any(|extension| rel_path.ends_with(extension))
}
