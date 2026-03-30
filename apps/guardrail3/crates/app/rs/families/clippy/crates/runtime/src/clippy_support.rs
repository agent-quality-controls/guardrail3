use guardrail3_domain_modules::clippy as policy;
#[derive(Debug)]
pub struct ThresholdExpectation {
    pub(crate) key: &'static str,
    pub(crate) expected: i64,
}

#[derive(Debug, Clone)]
pub struct BanEntry {
    pub(crate) path: String,
    pub(crate) reason: Option<String>,
    pub(crate) is_plain_string: bool,
}

#[derive(Debug, Clone)]
pub struct BanSectionFacts {
    pub(crate) entries: Vec<BanEntry>,
    pub(crate) malformed_messages: Vec<String>,
}

#[derive(Debug)]
pub enum IntegerSetting<'a> {
    Missing,
    WrongType(&'a toml::Value),
    Value(i64),
}

#[derive(Debug)]
pub enum BoolSetting<'a> {
    Missing,
    WrongType(&'a toml::Value),
    Value(bool),
}

pub const EXPECTED_MACRO_BANS: &[&str] = guardrail3_domain_modules::clippy::EXPECTED_MACRO_BANS;
pub const ALLOW_DBG_IN_TESTS: bool = policy::ALLOW_DBG_IN_TESTS;
pub const ALLOW_EXPECT_IN_TESTS: bool = policy::ALLOW_EXPECT_IN_TESTS;
pub const ALLOW_PANIC_IN_TESTS: bool = policy::ALLOW_PANIC_IN_TESTS;
pub const ALLOW_PRINT_IN_TESTS: bool = policy::ALLOW_PRINT_IN_TESTS;
pub const ALLOW_UNWRAP_IN_TESTS: bool = policy::ALLOW_UNWRAP_IN_TESTS;

pub const THRESHOLD_EXPECTATIONS: &[ThresholdExpectation] = &[
    ThresholdExpectation {
        key: "max-struct-bools",
        expected: policy::MAX_STRUCT_BOOLS,
    },
    ThresholdExpectation {
        key: "max-fn-params-bools",
        expected: policy::MAX_FN_PARAMS_BOOLS,
    },
    ThresholdExpectation {
        key: "too-many-lines-threshold",
        expected: policy::TOO_MANY_LINES_THRESHOLD,
    },
    ThresholdExpectation {
        key: "too-many-arguments-threshold",
        expected: policy::TOO_MANY_ARGUMENTS_THRESHOLD,
    },
    ThresholdExpectation {
        key: "excessive-nesting-threshold",
        expected: policy::EXCESSIVE_NESTING_THRESHOLD,
    },
    ThresholdExpectation {
        key: "cognitive-complexity-threshold",
        expected: policy::COGNITIVE_COMPLEXITY_THRESHOLD,
    },
    ThresholdExpectation {
        key: "type-complexity-threshold",
        expected: policy::TYPE_COMPLEXITY_THRESHOLD,
    },
];

pub const EXPECTED_METHOD_BANS: &[&str] = policy::SERVICE_METHOD_PATHS;
pub const EXPECTED_TYPE_BANS: &[&str] = policy::BASE_TYPE_PATHS;
pub const EXPECTED_LIBRARY_GLOBAL_STATE_TYPES: &[&str] = policy::LIBRARY_EXTRA_TYPE_PATHS;

pub fn build_clippy_toml(
    profile: &str,
    allow_global_state: bool,
    garde_enabled: bool,
    extra_method_bans: &str,
    extra_type_bans: &str,
) -> String {
    policy::build_clippy_toml(
        profile,
        allow_global_state,
        garde_enabled,
        extra_method_bans,
        extra_type_bans,
    )
}

pub fn library_profile_type_paths() -> Vec<&'static str> {
    policy::library_profile_type_paths()
}

pub fn service_profile_type_paths() -> Vec<&'static str> {
    policy::service_profile_type_paths()
}

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
    let mut bans = expected_required_type_bans(garde_enabled);
    if profile_name == Some("library") {
        bans.extend(EXPECTED_LIBRARY_GLOBAL_STATE_TYPES);
    }
    bans
}

pub fn expected_required_type_bans(garde_enabled: bool) -> Vec<&'static str> {
    let mut bans = EXPECTED_TYPE_BANS.to_vec();
    if !garde_enabled {
        bans.retain(|path| !GARDE_TYPE_BANS.contains(path));
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

pub fn parse_ban_section(parsed: &toml::Value, key: &str) -> BanSectionFacts {
    let Some(value) = parsed.get(key) else {
        return BanSectionFacts {
            entries: Vec::new(),
            malformed_messages: Vec::new(),
        };
    };

    let Some(entries) = value.as_array() else {
        return BanSectionFacts {
            entries: Vec::new(),
            malformed_messages: vec![format!(
                "`{key}` must be an array, found {}.",
                value_kind(value)
            )],
        };
    };

    let mut parsed_entries = Vec::new();
    let mut malformed_messages = Vec::new();

    for (index, entry) in entries.iter().enumerate() {
        match entry {
            toml::Value::String(path) => parsed_entries.push(BanEntry {
                path: path.clone(),
                reason: None,
                is_plain_string: true,
            }),
            toml::Value::Table(table) => match table.get("path") {
                Some(toml::Value::String(path)) => {
                    if let Some(reason) = table.get("reason") {
                        if !reason.is_str() {
                            malformed_messages.push(format!(
                                "`{key}[{index}].reason` must be a string when present, found {}.",
                                value_kind(reason)
                            ));
                            continue;
                        }
                    }
                    parsed_entries.push(BanEntry {
                        path: path.to_owned(),
                        reason: table
                            .get("reason")
                            .and_then(toml::Value::as_str)
                            .map(str::to_owned),
                        is_plain_string: false,
                    });
                }
                Some(path) => malformed_messages.push(format!(
                    "`{key}[{index}].path` must be a string, found {}.",
                    value_kind(path)
                )),
                None => malformed_messages.push(format!(
                    "`{key}[{index}]` must contain a string `path` field."
                )),
            },
            other => malformed_messages.push(format!(
                "`{key}[{index}]` must be a string or table, found {}.",
                value_kind(other)
            )),
        }
    }

    BanSectionFacts {
        entries: parsed_entries,
        malformed_messages,
    }
}

pub fn ban_paths(parsed: &toml::Value, key: &str) -> Vec<String> {
    parse_ban_section(parsed, key)
        .entries
        .into_iter()
        .map(|entry| entry.path)
        .collect()
}

pub fn integer_setting<'a>(parsed: &'a toml::Value, key: &str) -> IntegerSetting<'a> {
    match parsed.get(key) {
        None => IntegerSetting::Missing,
        Some(value) => match value.as_integer() {
            Some(actual) => IntegerSetting::Value(actual),
            None => IntegerSetting::WrongType(value),
        },
    }
}

pub fn bool_setting<'a>(parsed: &'a toml::Value, key: &str) -> BoolSetting<'a> {
    match parsed.get(key) {
        None => BoolSetting::Missing,
        Some(value) => match value.as_bool() {
            Some(actual) => BoolSetting::Value(actual),
            None => BoolSetting::WrongType(value),
        },
    }
}

pub fn threshold_value(parsed: &toml::Value, key: &str) -> Option<i64> {
    match integer_setting(parsed, key) {
        IntegerSetting::Value(value) => Some(value),
        IntegerSetting::Missing | IntegerSetting::WrongType(_) => None,
    }
}

pub fn display_macro_name(path: &str) -> &str {
    path.rsplit("::").next().unwrap_or(path)
}

pub fn known_top_level_keys() -> Vec<&'static str> {
    policy::THRESHOLD_VALUES
        .iter()
        .map(|(key, _)| *key)
        .collect()
}

pub fn managed_non_threshold_keys() -> Vec<&'static str> {
    vec![
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
        "avoid-breaking-exported-api",
        "allow-dbg-in-tests",
        "allow-expect-in-tests",
        "allow-panic-in-tests",
        "allow-print-in-tests",
        "allow-unwrap-in-tests",
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

pub fn value_kind(value: &toml::Value) -> &'static str {
    match value {
        toml::Value::String(_) => "string",
        toml::Value::Integer(_) => "integer",
        toml::Value::Float(_) => "float",
        toml::Value::Boolean(_) => "bool",
        toml::Value::Datetime(_) => "datetime",
        toml::Value::Array(_) => "array",
        toml::Value::Table(_) => "table",
    }
}
