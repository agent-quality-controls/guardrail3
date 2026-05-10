/// MAX STRUCT BOOLS const.
pub(crate) const MAX_STRUCT_BOOLS: i64 = 3;
/// MAX FN PARAMS BOOLS const.
pub(crate) const MAX_FN_PARAMS_BOOLS: i64 = 3;
/// TOO MANY LINES THRESHOLD const.
pub(crate) const TOO_MANY_LINES_THRESHOLD: i64 = 75;
/// TOO MANY ARGUMENTS THRESHOLD const.
pub(crate) const TOO_MANY_ARGUMENTS_THRESHOLD: i64 = 7;
/// EXCESSIVE NESTING THRESHOLD const.
pub(crate) const EXCESSIVE_NESTING_THRESHOLD: i64 = 4;
/// COGNITIVE COMPLEXITY THRESHOLD const.
pub(crate) const COGNITIVE_COMPLEXITY_THRESHOLD: i64 = 15;
/// TYPE COMPLEXITY THRESHOLD const.
pub(crate) const TYPE_COMPLEXITY_THRESHOLD: i64 = 75;

/// ALLOW DBG IN TESTS const.
pub(crate) const ALLOW_DBG_IN_TESTS: bool = false;
/// ALLOW EXPECT IN TESTS const.
pub(crate) const ALLOW_EXPECT_IN_TESTS: bool = true;
/// ALLOW PANIC IN TESTS const.
pub(crate) const ALLOW_PANIC_IN_TESTS: bool = false;
/// ALLOW PRINT IN TESTS const.
pub(crate) const ALLOW_PRINT_IN_TESTS: bool = false;
/// ALLOW UNWRAP IN TESTS const.
pub(crate) const ALLOW_UNWRAP_IN_TESTS: bool = false;

/// EXPECTED MACRO BANS const.
pub(crate) const EXPECTED_MACRO_BANS: &[&str] = &[
    "std::println",
    "std::eprintln",
    "std::dbg",
    "std::todo",
    "std::unimplemented",
];

/// SERVICE METHOD PATHS const.
pub(crate) const SERVICE_METHOD_PATHS: &[&str] = &[
    "std::env::var",
    "std::env::var_os",
    "std::env::vars",
    "std::env::set_var",
    "std::env::remove_var",
    "std::process::exit",
    "std::process::abort",
    "std::process::Command::new",
    "std::thread::sleep",
    "std::fs::read_to_string",
    "std::fs::read",
    "std::fs::read_dir",
    "std::fs::read_link",
    "std::fs::write",
    "std::fs::remove_file",
    "std::fs::remove_dir_all",
    "std::fs::create_dir_all",
    "std::fs::rename",
    "std::fs::copy",
    "std::fs::metadata",
    "std::fs::symlink_metadata",
    "std::fs::canonicalize",
    "std::fs::set_permissions",
    "std::fs::hard_link",
    "reqwest::Client::new",
    "reqwest::Client::builder",
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

/// BASE TYPE PATHS const.
pub(crate) const BASE_TYPE_PATHS: &[&str] = &[
    "std::collections::HashMap",
    "std::collections::HashSet",
    "std::sync::Mutex",
    "std::sync::RwLock",
    "std::fs::File",
    "std::any::Any",
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

/// LIBRARY EXTRA TYPE PATHS const.
pub(crate) const LIBRARY_EXTRA_TYPE_PATHS: &[&str] = &[
    "std::sync::LazyLock",
    "std::sync::OnceLock",
    "once_cell::sync::Lazy",
    "once_cell::sync::OnceCell",
];
