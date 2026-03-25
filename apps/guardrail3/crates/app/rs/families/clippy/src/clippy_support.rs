use guardrail3_domain_modules::clippy::{
    ALLOW_DBG_IN_TESTS, ALLOW_PRINT_IN_TESTS, AVOID_BREAKING_EXPORTED_API, BASE_TYPE_PATHS,
    COGNITIVE_COMPLEXITY_THRESHOLD, EXCESSIVE_NESTING_THRESHOLD, LIBRARY_EXTRA_TYPE_PATHS,
    MAX_FN_PARAMS_BOOLS, MAX_STRUCT_BOOLS, SERVICE_METHOD_PATHS, THRESHOLD_VALUES,
    TOO_MANY_ARGUMENTS_THRESHOLD, TOO_MANY_LINES_THRESHOLD, TYPE_COMPLEXITY_THRESHOLD,
};
pub struct ThresholdExpectation {
    pub key: &'static str,
    pub expected: i64,
}

#[derive(Debug, Clone)]
pub struct BanEntry {
    pub path: String,
    pub reason: Option<String>,
    pub is_plain_string: bool,
}

pub const EXPECTED_MACRO_BANS: &[&str] = guardrail3_domain_modules::clippy::EXPECTED_MACRO_BANS;

pub const THRESHOLD_EXPECTATIONS: &[ThresholdExpectation] = &[
    ThresholdExpectation {
        key: "max-struct-bools",
        expected: MAX_STRUCT_BOOLS,
    },
    ThresholdExpectation {
        key: "max-fn-params-bools",
        expected: MAX_FN_PARAMS_BOOLS,
    },
    ThresholdExpectation {
        key: "too-many-lines-threshold",
        expected: TOO_MANY_LINES_THRESHOLD,
    },
    ThresholdExpectation {
        key: "too-many-arguments-threshold",
        expected: TOO_MANY_ARGUMENTS_THRESHOLD,
    },
    ThresholdExpectation {
        key: "excessive-nesting-threshold",
        expected: EXCESSIVE_NESTING_THRESHOLD,
    },
    ThresholdExpectation {
        key: "cognitive-complexity-threshold",
        expected: COGNITIVE_COMPLEXITY_THRESHOLD,
    },
    ThresholdExpectation {
        key: "type-complexity-threshold",
        expected: TYPE_COMPLEXITY_THRESHOLD,
    },
];

pub const EXPECTED_METHOD_BANS: &[&str] = SERVICE_METHOD_PATHS;
pub const EXPECTED_TYPE_BANS: &[&str] = BASE_TYPE_PATHS;
pub const EXPECTED_LIBRARY_GLOBAL_STATE_TYPES: &[&str] = LIBRARY_EXTRA_TYPE_PATHS;

const GARDE_METHOD_BANS: &[&str] = &[
    "serde_json::from_str",
    "serde_json::from_slice",
    "serde_json::from_value",
    "serde_json::from_reader",
    "reqwest::Response::json",
    "toml::from_str",
    "serde_yaml::from_str",
    "serde_yaml::from_reader",
    "serde_qs::from_str",
    "serde_qs::from_bytes",
    "serde_urlencoded::from_str",
    "serde_urlencoded::from_bytes",
    "serde_urlencoded::from_reader",
    "ciborium::from_reader",
    "ciborium::de::from_reader",
    "rmp_serde::from_slice",
    "rmp_serde::from_read",
    "rmp_serde::decode::from_slice",
    "rmp_serde::decode::from_read",
    "bincode::deserialize",
    "bincode::deserialize_from",
    "bincode::serde::decode_from_slice",
    "bincode::serde::decode_from_reader",
    "csv::Reader::deserialize",
    "csv::StringRecord::deserialize",
    "csv::ByteRecord::deserialize",
    "serde_xml_rs::from_str",
    "serde_xml_rs::from_reader",
    "quick_xml::de::from_str",
    "quick_xml::de::from_reader",
    "ron::from_str",
    "ron::de::from_str",
    "serde_cbor::from_slice",
    "serde_cbor::from_reader",
    "postcard::from_bytes",
    "flexbuffers::from_slice",
    "serde_json::Deserializer::from_str",
    "serde_json::Deserializer::from_slice",
    "serde_json::Deserializer::from_reader",
    "toml_edit::de::from_str",
    "toml_edit::de::from_slice",
    "toml_edit::de::from_document",
    "config::Config::try_deserialize",
    "figment::Figment::extract",
];

const GARDE_TYPE_BANS: &[&str] = &[
    "axum::extract::Json",
    "axum::Json",
    "axum::extract::Query",
    "axum::extract::Form",
    "axum::extract::Path",
    "axum::extract::Multipart",
    "axum::extract::ConnectInfo",
    "axum_extra::extract::CookieJar",
    "axum_extra::extract::cookie::Cookie",
    "axum_extra::extract::TypedHeader",
    "axum_extra::extract::JsonDeserializer",
    "axum_extra::extract::JsonLines",
    "axum_extra::extract::Protobuf",
    "axum_extra::extract::Cbor",
    "axum_extra::extract::MsgPack",
];

pub fn expected_type_bans(profile_name: Option<&str>, garde_enabled: bool) -> Vec<&'static str> {
    let mut bans = EXPECTED_TYPE_BANS.to_vec();
    if !garde_enabled {
        bans.retain(|path| !GARDE_TYPE_BANS.contains(path));
    }
    if profile_name == Some("library") {
        bans.extend(EXPECTED_LIBRARY_GLOBAL_STATE_TYPES);
    }
    bans
}

pub fn expected_method_bans(garde_enabled: bool) -> Vec<&'static str> {
    let mut bans = EXPECTED_METHOD_BANS.to_vec();
    if !garde_enabled {
        bans.retain(|path| !GARDE_METHOD_BANS.contains(path));
    }
    bans
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
    THRESHOLD_VALUES.iter().map(|(key, _)| *key).collect()
}

pub fn managed_non_threshold_keys() -> Vec<&'static str> {
    vec![
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
        "avoid-breaking-exported-api",
        "allow-dbg-in-tests",
        "allow-print-in-tests",
    ]
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

pub fn normalized_key_distance(a: &str, b: &str) -> usize {
    let a = a.replace(['-', '_'], "");
    let b = b.replace(['-', '_'], "");
    levenshtein(a.as_bytes(), b.as_bytes())
}

pub fn levenshtein(a: &[u8], b: &[u8]) -> usize {
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }

    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr = vec![0usize; b.len() + 1];

    for (i, a_byte) in a.iter().enumerate() {
        curr[0] = i + 1;
        for (j, b_byte) in b.iter().enumerate() {
            let cost = usize::from(a_byte != b_byte);
            curr[j + 1] = (curr[j] + 1).min(prev[j + 1] + 1).min(prev[j] + cost);
        }
        prev.clone_from(&curr);
    }

    prev[b.len()]
}
