use super::content::is_source_module_file;
use super::prelude::*;
use super::constants::*;

pub(super) fn rule_setting_has_expected_module_globs(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    key: &str,
    expected_sources: &[String],
) -> bool {
    let expected = expected_module_globs(expected_sources);
    !expected.is_empty()
        && string_arrays_match_as_sets(&string_array_option(setting, key), &expected)
}

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

pub(super) fn string_arrays_match_as_sets(left: &[String], right: &[String]) -> bool {
    BTreeSet::from_iter(left.iter().map(|value| normalize_glob(value)))
        == BTreeSet::from_iter(right.iter().map(|value| normalize_glob(value)))
}

pub(super) fn rule_setting_has_inline_public_content_policy(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    let Some(object) = setting
        .options
        .first()
        .and_then(serde_json::Value::as_object)
    else {
        return false;
    };

    object.len() == 10
        && object_string_value(object.get("framework")) == Some("react")
        && object_string_value(object.get("mode")) == Some("all")
        && object_string_value(object.get("message")) == Some(INLINE_PUBLIC_CONTENT_MESSAGE)
        && object_bool_value(object.get("should-validate-template")) == Some(true)
        && object_has_exact_string_arrays(
            object.get("words"),
            "include",
            &[],
            "exclude",
            &["[0-9!-/:-@[-`{-~]+", "[A-Z_-]+"],
        )
        && object_has_exact_string_arrays(
            object.get("jsx-components"),
            "include",
            &[],
            "exclude",
            &[],
        )
        && object_has_exact_string_arrays(
            object.get("jsx-attributes"),
            "include",
            &[],
            "exclude",
            &[
                "as",
                "class",
                "className",
                "color",
                "data-.+",
                "height",
                "href",
                "id",
                "intent",
                "key",
                "name",
                "rel",
                "role",
                "size",
                "slot",
                "src",
                "style",
                "styleName",
                "target",
                "tone",
                "type",
                "variant",
                "width",
                "aria-hidden",
            ],
        )
        && object_has_exact_string_arrays(
            object.get("callees"),
            "include",
            &[],
            "exclude",
            &[
                "require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL",
            ],
        )
        && object_has_exact_string_arrays(
            object.get("object-properties"),
            "include",
            &[],
            "exclude",
            &["[A-Z_-]+"],
        )
        && object_has_exact_string_arrays(
            object.get("class-properties"),
            "include",
            &[],
            "exclude",
            &["displayName"],
        )
}

pub(super) fn object_has_exact_string_arrays(
    value: Option<&serde_json::Value>,
    first_key: &str,
    first_expected: &[&str],
    second_key: &str,
    second_expected: &[&str],
) -> bool {
    let Some(object) = value.and_then(serde_json::Value::as_object) else {
        return false;
    };

    object.len() == 2
        && string_array_exactly(object.get(first_key), first_expected)
        && string_array_exactly(object.get(second_key), second_expected)
}

pub(super) fn string_array_exactly(value: Option<&serde_json::Value>, expected: &[&str]) -> bool {
    let Some(values) = value.and_then(serde_json::Value::as_array) else {
        return false;
    };

    values.len() == expected.len()
        && values
            .iter()
            .zip(expected.iter().copied())
            .all(|(value, expected)| value.as_str() == Some(expected))
}

pub(super) fn object_string_value(option: Option<&serde_json::Value>) -> Option<&str> {
    option.and_then(serde_json::Value::as_str)
}

pub(super) fn object_bool_value(option: Option<&serde_json::Value>) -> Option<bool> {
    option.and_then(serde_json::Value::as_bool)
}

pub(super) fn has_non_empty_string_array_option(option: Option<&serde_json::Value>) -> bool {
    option
        .and_then(serde_json::Value::as_array)
        .is_some_and(|values| {
            !values.is_empty()
                && values
                    .iter()
                    .all(|value| value.as_str().is_some_and(|text| !text.trim().is_empty()))
        })
}

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

pub(super) fn rule_setting_option_globs_are_valid(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        non_empty_string_array_option(object.get(option_name))
            .is_some_and(|patterns| globs_are_valid(&patterns))
    })
}

pub(super) fn first_option_object(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> Option<&serde_json::Map<String, serde_json::Value>> {
    setting
        .options
        .first()
        .and_then(serde_json::Value::as_object)
}

pub(super) fn rule_setting_is_error(setting: &eslint_config_parser::types::EslintRuleSetting) -> bool {
    setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
}

pub(super) fn non_empty_string_array_option(option: Option<&serde_json::Value>) -> Option<Vec<&str>> {
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

pub(super) fn string_array_option(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
) -> Vec<String> {
    first_option_object(setting)
        .and_then(|object| object.get(option_name))
        .and_then(serde_json::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn all_paths_match_globs(patterns: &[&str], candidate_paths: &[String]) -> bool {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let Ok(glob) = Glob::new(&normalize_glob(pattern)) else {
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

pub(super) fn globs_are_valid(patterns: &[&str]) -> bool {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let Ok(glob) = Glob::new(&normalize_glob(pattern)) else {
            return false;
        };
        let _ = builder.add(glob);
    }

    builder.build().is_ok()
}

pub(super) fn glob_set_from_strings(patterns: &[String]) -> Result<GlobSet, globset::Error> {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let _ = builder.add(Glob::new(&normalize_glob(pattern))?);
    }

    builder.build()
}

pub(super) fn normalize_glob(value: &str) -> String {
    let mut normalized = value.replace('\\', "/");
    while normalized.contains("//") {
        normalized = normalized.replace("//", "/");
    }
    normalized.trim_start_matches("./").to_owned()
}
