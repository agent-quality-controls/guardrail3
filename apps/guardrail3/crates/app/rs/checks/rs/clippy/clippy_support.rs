use crate::domain::modules::clippy::{
    ALLOW_DBG_IN_TESTS, ALLOW_PRINT_IN_TESTS, AVOID_BREAKING_EXPORTED_API, BASE_TYPE_PATHS,
    COGNITIVE_COMPLEXITY_THRESHOLD, EXCESSIVE_NESTING_THRESHOLD, LIBRARY_EXTRA_TYPE_PATHS,
    MAX_FN_PARAMS_BOOLS, MAX_STRUCT_BOOLS, SERVICE_METHOD_PATHS, THRESHOLD_VALUES,
    TOO_MANY_ARGUMENTS_THRESHOLD, TOO_MANY_LINES_THRESHOLD, TYPE_COMPLEXITY_THRESHOLD,
};

pub struct ThresholdExpectation {
    pub id: &'static str,
    pub key: &'static str,
    pub expected: i64,
}

#[derive(Debug, Clone)]
pub struct BanEntry {
    pub path: String,
    pub reason: Option<String>,
    pub is_plain_string: bool,
}

pub const EXPECTED_MACRO_BANS: &[&str] = crate::domain::modules::clippy::EXPECTED_MACRO_BANS;

pub const THRESHOLD_EXPECTATIONS: &[ThresholdExpectation] = &[
    ThresholdExpectation {
        id: "RS-CLIPPY-02",
        key: "max-struct-bools",
        expected: MAX_STRUCT_BOOLS,
    },
    ThresholdExpectation {
        id: "RS-CLIPPY-03",
        key: "max-fn-params-bools",
        expected: MAX_FN_PARAMS_BOOLS,
    },
    ThresholdExpectation {
        id: "RS-CLIPPY-09",
        key: "too-many-lines-threshold",
        expected: TOO_MANY_LINES_THRESHOLD,
    },
    ThresholdExpectation {
        id: "RS-CLIPPY-10",
        key: "too-many-arguments-threshold",
        expected: TOO_MANY_ARGUMENTS_THRESHOLD,
    },
    ThresholdExpectation {
        id: "RS-CLIPPY-11",
        key: "excessive-nesting-threshold",
        expected: EXCESSIVE_NESTING_THRESHOLD,
    },
    ThresholdExpectation {
        id: "RS-CLIPPY-21",
        key: "cognitive-complexity-threshold",
        expected: COGNITIVE_COMPLEXITY_THRESHOLD,
    },
    ThresholdExpectation {
        id: "RS-CLIPPY-22",
        key: "type-complexity-threshold",
        expected: TYPE_COMPLEXITY_THRESHOLD,
    },
];

pub const EXPECTED_METHOD_BANS: &[&str] = SERVICE_METHOD_PATHS;
pub const EXPECTED_TYPE_BANS: &[&str] = BASE_TYPE_PATHS;
pub const EXPECTED_LIBRARY_GLOBAL_STATE_TYPES: &[&str] = LIBRARY_EXTRA_TYPE_PATHS;

pub fn expected_type_bans(profile_name: Option<&str>) -> Vec<&'static str> {
    let mut bans = EXPECTED_TYPE_BANS.to_vec();
    if profile_name == Some("library") {
        bans.extend(EXPECTED_LIBRARY_GLOBAL_STATE_TYPES);
    }
    bans
}

pub fn expected_method_bans() -> Vec<&'static str> {
    EXPECTED_METHOD_BANS.to_vec()
}

pub fn parse_ban_entries(parsed: &toml::Value, key: &str) -> Vec<BanEntry> {
    parsed
        .get(key)
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| match entry {
                    toml::Value::String(path) => Some(BanEntry {
                        path: path.clone(),
                        reason: None,
                        is_plain_string: true,
                    }),
                    toml::Value::Table(table) => table
                        .get("path")
                        .and_then(toml::Value::as_str)
                        .map(|path| BanEntry {
                            path: path.to_owned(),
                            reason: table
                                .get("reason")
                                .and_then(toml::Value::as_str)
                                .map(str::to_owned),
                            is_plain_string: false,
                        }),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn ban_paths(parsed: &toml::Value, key: &str) -> Vec<String> {
    parse_ban_entries(parsed, key)
        .into_iter()
        .map(|entry| entry.path)
        .collect()
}

pub fn threshold_value(parsed: &toml::Value, key: &str) -> Option<i64> {
    parsed.get(key).and_then(toml::Value::as_integer)
}

pub fn known_top_level_keys() -> Vec<&'static str> {
    let mut keys: Vec<_> = THRESHOLD_VALUES.iter().map(|(key, _)| *key).collect();
    keys.extend([
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
        "avoid-breaking-exported-api",
        "allow-dbg-in-tests",
        "allow-print-in-tests",
        "msrv",
        "allowed-duplicate-crates",
        "arithmetic-side-effects-allowed",
        "allow-expect-in-tests",
        "allow-unwrap-in-tests",
        "doc-valid-idents",
        "await-holding-invalid-types",
    ]);
    keys
}

pub fn is_placeholder_reason(reason: &str) -> bool {
    let normalized = reason.trim().to_ascii_lowercase();
    normalized.is_empty()
        || normalized.len() < 10
        || matches!(
            normalized.as_str(),
            "todo" | "fixme" | "fix later" | "tbd" | "..." | "reason"
        )
}

pub fn expected_bool_value(key: &str) -> Option<bool> {
    match key {
        "avoid-breaking-exported-api" => Some(AVOID_BREAKING_EXPORTED_API),
        "allow-dbg-in-tests" => Some(ALLOW_DBG_IN_TESTS),
        "allow-print-in-tests" => Some(ALLOW_PRINT_IN_TESTS),
        _ => None,
    }
}
