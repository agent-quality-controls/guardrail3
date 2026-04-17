use clippy_toml_parser::types::{
    ClippyBanEntry as ParserBanEntry, ClippyBanSection as ParserBanSection,
    ClippyBoolSetting as ParserBoolSetting, ClippyToml, ClippyTomlDocument,
};
use g3rs_clippy_types::{
    G3RsClippyCargoMemberState, G3RsClippyCargoRootState, G3RsClippyConfigChecksInput,
    G3RsClippyConfigState, G3RsClippyRustPolicyState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_rs_toml_parser::types::RustProfile;

#[derive(Debug)]
pub(crate) struct ThresholdExpectation {
    pub(crate) key: &'static str,
}

#[derive(Debug, Clone)]
pub(crate) struct BanEntry {
    pub(crate) path: String,
    pub(crate) reason: Option<String>,
    pub(crate) is_plain_string: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct BanSectionFacts {
    pub(crate) entries: Vec<BanEntry>,
    pub(crate) malformed_messages: Vec<String>,
}

#[derive(Debug)]
pub(crate) enum BoolSetting<'a> {
    Missing,
    WrongType(&'a toml::Value),
    Value(bool),
}

pub(crate) const EXPECTED_MACRO_BANS: &[&str] = crate::baseline::EXPECTED_MACRO_BANS;
pub(crate) const EXPECTED_LIBRARY_GLOBAL_STATE_TYPES: &[&str] =
    crate::baseline::LIBRARY_EXTRA_TYPE_PATHS;
const THRESHOLD_EXPECTATIONS: &[ThresholdExpectation] = &[
    ThresholdExpectation {
        key: "max-struct-bools",
    },
    ThresholdExpectation {
        key: "max-fn-params-bools",
    },
    ThresholdExpectation {
        key: "too-many-lines-threshold",
    },
    ThresholdExpectation {
        key: "too-many-arguments-threshold",
    },
    ThresholdExpectation {
        key: "excessive-nesting-threshold",
    },
    ThresholdExpectation {
        key: "cognitive-complexity-threshold",
    },
    ThresholdExpectation {
        key: "type-complexity-threshold",
    },
];

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

pub(crate) fn check_threshold(
    id: &str,
    clippy_rel_path: &str,
    key: &str,
    actual: Option<u64>,
    expected: i64,
    results: &mut Vec<G3CheckResult>,
) {
    match actual {
        Some(actual) if i64::try_from(actual).ok() == Some(expected) => {
            results.push(
                G3CheckResult::new(
                    id.to_owned(),
                    G3Severity::Info,
                    format!("{key} correct"),
                    format!("{key} = {expected}"),
                    Some(clippy_rel_path.to_owned()),
                    None,
                )
                .into_inventory(),
            );
        }
        Some(actual) => {
            results.push(G3CheckResult::new(
                id.to_owned(),
                G3Severity::Error,
                format!("{key} wrong value"),
                format!(
                    "Expected {expected}, got {actual}. Set `{key} = {expected}` in clippy.toml."
                ),
                Some(clippy_rel_path.to_owned()),
                None,
            ));
        }
        None => {
            results.push(G3CheckResult::new(
                id.to_owned(),
                G3Severity::Error,
                format!("{key} missing"),
                format!("Add `{key} = {expected}` to clippy.toml."),
                Some(clippy_rel_path.to_owned()),
                None,
            ));
        }
    }
}

pub(crate) fn has_matching_waiver(
    input: &G3RsClippyConfigChecksInput,
    rule: &str,
    selector: &str,
) -> bool {
    input.waivers.iter().any(|waiver| {
        waiver.rule == rule
            && waiver.file == input.clippy_rel_path
            && waiver.selector == selector
    })
}

pub(crate) fn typed_clippy(input: &G3RsClippyConfigChecksInput) -> Option<&ClippyToml> {
    match &input.clippy {
        G3RsClippyConfigState::Parsed(document) => clippy_toml_parser::typed(document),
        G3RsClippyConfigState::Unreadable { .. }
        | G3RsClippyConfigState::ParseError { .. } => None,
    }
}

pub(crate) fn clippy_document(
    input: &G3RsClippyConfigChecksInput,
) -> Option<&ClippyTomlDocument> {
    match &input.clippy {
        G3RsClippyConfigState::Parsed(document) => Some(document),
        G3RsClippyConfigState::Unreadable { .. } | G3RsClippyConfigState::ParseError { .. } => None,
    }
}

pub(crate) fn typed_parse_error(input: &G3RsClippyConfigChecksInput) -> Option<&str> {
    match &input.clippy {
        G3RsClippyConfigState::Parsed(document) => clippy_toml_parser::parse_error_reason(document),
        G3RsClippyConfigState::Unreadable { .. } | G3RsClippyConfigState::ParseError { .. } => None,
    }
}

pub(crate) fn raw_parse_error(input: &G3RsClippyConfigChecksInput) -> Option<&str> {
    match &input.clippy {
        G3RsClippyConfigState::Unreadable { reason }
        | G3RsClippyConfigState::ParseError { reason } => Some(reason),
        G3RsClippyConfigState::Parsed(_) => None,
    }
}

pub(crate) fn rust_policy_valid(input: &G3RsClippyConfigChecksInput) -> bool {
    matches!(
        input.rust_policy,
        G3RsClippyRustPolicyState::Missing | G3RsClippyRustPolicyState::Parsed { .. }
    )
}

pub(crate) fn rust_profile(input: &G3RsClippyConfigChecksInput) -> Option<RustProfile> {
    match &input.rust_policy {
        G3RsClippyRustPolicyState::Parsed { profile, .. } => *profile,
        G3RsClippyRustPolicyState::Missing
        | G3RsClippyRustPolicyState::Unreadable { .. }
        | G3RsClippyRustPolicyState::ParseError { .. } => None,
    }
}

pub(crate) fn garde_enabled(input: &G3RsClippyConfigChecksInput) -> bool {
    match &input.rust_policy {
        G3RsClippyRustPolicyState::Parsed { garde_enabled, .. } => *garde_enabled,
        G3RsClippyRustPolicyState::Missing => true,
        G3RsClippyRustPolicyState::Unreadable { .. }
        | G3RsClippyRustPolicyState::ParseError { .. } => true,
    }
}

pub(crate) fn rust_policy_failure(input: &G3RsClippyConfigChecksInput) -> Option<(&str, &str)> {
    match &input.rust_policy {
        G3RsClippyRustPolicyState::Unreadable { rel_path, reason }
        | G3RsClippyRustPolicyState::ParseError { rel_path, reason } => Some((rel_path, reason)),
        G3RsClippyRustPolicyState::Missing | G3RsClippyRustPolicyState::Parsed { .. } => None,
    }
}

pub(crate) fn rust_policy_rel_path(input: &G3RsClippyConfigChecksInput) -> Option<&str> {
    match &input.rust_policy {
        G3RsClippyRustPolicyState::Unreadable { rel_path, .. }
        | G3RsClippyRustPolicyState::ParseError { rel_path, .. }
        | G3RsClippyRustPolicyState::Parsed { rel_path, .. } => Some(rel_path),
        G3RsClippyRustPolicyState::Missing => None,
    }
}

pub(crate) fn published_library_policy(input: &G3RsClippyConfigChecksInput) -> bool {
    let G3RsClippyCargoRootState::Parsed { cargo: root_cargo, .. } = &input.cargo_root else {
        return false;
    };
    let Some(root_typed) = cargo_toml_parser::document::typed(root_cargo) else {
        return false;
    };
    let workspace_publish = root_typed
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.package.as_ref())
        .and_then(|package| package.publish.as_ref());

    if manifest_publishable(root_typed, workspace_publish) {
        return true;
    }

    input
        .cargo_workspace_members
        .iter()
        .filter_map(|member| match member {
            G3RsClippyCargoMemberState::Parsed { cargo, .. } => cargo_toml_parser::document::typed(cargo),
            G3RsClippyCargoMemberState::Unreadable { .. }
            | G3RsClippyCargoMemberState::ParseError { .. } => None,
        })
        .any(|member| manifest_publishable(member, workspace_publish))
}

pub(crate) fn relaxation_message(key: &str, expected: bool, actual: Option<bool>) -> String {
    let policy = match key {
        "allow-dbg-in-tests" | "allow-print-in-tests" => {
            "Tests should stay quiet and deterministic."
        }
        "allow-expect-in-tests" => {
            "Tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`."
        }
        "allow-panic-in-tests" => "panic!() must remain banned in tests.",
        "allow-unwrap-in-tests" => "unwrap() must remain banned in tests.",
        _ => "Managed test relaxation keys must match the hardened clippy policy.",
    };

    match actual {
        Some(actual) => format!("`{key}` must be `{expected}`; found `{actual}`. {policy}"),
        None => format!("`{key}` must be set explicitly to `{expected}`. {policy}"),
    }
}

pub(crate) const fn allow_dbg_in_tests(clippy: &ClippyToml) -> Option<bool> {
    clippy.allow_dbg_in_tests
}

pub(crate) const fn allow_print_in_tests(clippy: &ClippyToml) -> Option<bool> {
    clippy.allow_print_in_tests
}

pub(crate) const fn allow_expect_in_tests(clippy: &ClippyToml) -> Option<bool> {
    clippy.allow_expect_in_tests
}

pub(crate) const fn allow_panic_in_tests(clippy: &ClippyToml) -> Option<bool> {
    clippy.allow_panic_in_tests
}

pub(crate) const fn allow_unwrap_in_tests(clippy: &ClippyToml) -> Option<bool> {
    clippy.allow_unwrap_in_tests
}

pub(crate) fn expected_required_type_bans(garde_enabled: bool) -> Vec<&'static str> {
    let mut bans = crate::baseline::BASE_TYPE_PATHS.to_vec();
    if !garde_enabled {
        bans.retain(|path| !GARDE_TYPE_BANS.contains(path));
    }
    bans
}

pub(crate) fn expected_type_bans(
    profile: Option<RustProfile>,
    garde_enabled: bool,
) -> Vec<&'static str> {
    let mut bans = expected_required_type_bans(garde_enabled);
    if profile == Some(RustProfile::Library) {
        bans.extend(EXPECTED_LIBRARY_GLOBAL_STATE_TYPES);
    }
    bans
}

pub(crate) fn expected_method_bans(garde_enabled: bool) -> Vec<&'static str> {
    let mut bans = crate::baseline::SERVICE_METHOD_PATHS.to_vec();
    if !garde_enabled {
        bans.retain(|path| !GARDE_METHOD_BANS.contains(path));
    }
    bans
}

pub(crate) fn parse_ban_section(
    document: &ClippyTomlDocument,
    key: &str,
) -> BanSectionFacts {
    let ParserBanSection {
        entries,
        malformed_messages,
    } = clippy_toml_parser::ban_section(document, key);
    BanSectionFacts {
        entries: entries
            .into_iter()
            .map(
                |ParserBanEntry {
                     path,
                     reason,
                     is_plain_string,
                 }| BanEntry {
                    path,
                    reason,
                    is_plain_string,
                },
            )
            .collect(),
        malformed_messages,
    }
}

pub(crate) fn bool_setting<'a>(
    document: &'a ClippyTomlDocument,
    key: &str,
) -> BoolSetting<'a> {
    match clippy_toml_parser::bool_setting(document, key) {
        ParserBoolSetting::Missing => BoolSetting::Missing,
        ParserBoolSetting::WrongType(value) => BoolSetting::WrongType(value),
        ParserBoolSetting::Value(actual) => BoolSetting::Value(actual),
    }
}

pub(crate) fn display_macro_name(path: &str) -> &str {
    path.rsplit("::").next().unwrap_or(path)
}

pub(crate) fn managed_non_threshold_keys() -> Vec<&'static str> {
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

pub(crate) fn known_top_level_keys() -> Vec<&'static str> {
    THRESHOLD_EXPECTATIONS
        .iter()
        .map(|expectation| expectation.key)
        .collect()
}

pub(crate) fn normalized_key_distance(a: &str, b: &str) -> usize {
    let a = a.replace(['-', '_'], "");
    let b = b.replace(['-', '_'], "");
    levenshtein(&a, &b)
}

pub(crate) fn value_kind(value: &toml::Value) -> &'static str {
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

fn manifest_publishable(
    cargo: &cargo_toml_parser::types::CargoToml,
    workspace_publish: Option<&cargo_toml_parser::types::VecStringOrBool>,
) -> bool {
    let Some(package) = cargo.package.as_ref().or(cargo.project.as_ref()) else {
        return false;
    };
    package_publishable(package, workspace_publish)
}

fn package_publishable(
    package: &cargo_toml_parser::types::PackageSection,
    workspace_publish: Option<&cargo_toml_parser::types::VecStringOrBool>,
) -> bool {
    match package.publish.as_ref() {
        None => true,
        Some(cargo_toml_parser::types::InheritableValue::Value(value)) => {
            publish_value_allows_publish(value)
        }
        Some(cargo_toml_parser::types::InheritableValue::Inherit(inheritance))
            if inheritance.workspace =>
        {
            workspace_publish.is_some_and(publish_value_allows_publish)
        }
        Some(cargo_toml_parser::types::InheritableValue::Inherit(_)) => false,
    }
}

fn publish_value_allows_publish(value: &cargo_toml_parser::types::VecStringOrBool) -> bool {
    match value {
        cargo_toml_parser::types::VecStringOrBool::Bool(flag) => *flag,
        cargo_toml_parser::types::VecStringOrBool::VecString(registries) => !registries.is_empty(),
    }
}

fn levenshtein(left: &str, right: &str) -> usize {
    let left_chars = left.chars().collect::<Vec<_>>();
    let right_chars = right.chars().collect::<Vec<_>>();
    let mut prev = (0..=right_chars.len()).collect::<Vec<_>>();

    for (i, left_char) in left_chars.iter().enumerate() {
        let mut current = vec![i + 1];
        for (j, right_char) in right_chars.iter().enumerate() {
            let cost = usize::from(left_char != right_char);
            let insertion = current[j] + 1;
            let deletion = prev[j + 1] + 1;
            let substitution = prev[j] + cost;
            current.push(insertion.min(deletion).min(substitution));
        }
        prev = current;
    }

    prev[right_chars.len()]
}
