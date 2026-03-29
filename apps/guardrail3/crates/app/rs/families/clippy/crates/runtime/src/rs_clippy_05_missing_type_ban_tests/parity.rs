use std::collections::BTreeSet;

use test_support::build_fixture_clippy_toml;

#[test]
fn generated_service_type_ban_set_matches_rule_baseline() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", ""))
            .expect("valid clippy TOML");
    let actual = parsed
        .get("disallowed-types")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("path").and_then(toml::Value::as_str))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let expected = [
        "std::collections::HashMap",
        "std::collections::HashSet",
        "std::sync::Mutex",
        "std::sync::RwLock",
        "std::fs::File",
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
        "std::any::Any",
    ]
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}

#[test]
fn generated_library_type_ban_set_matches_rule_baseline() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("library", false, true, "", ""))
        .expect("valid clippy TOML");
    let actual = parsed
        .get("disallowed-types")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("path").and_then(toml::Value::as_str))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let expected = [
        "std::collections::HashMap",
        "std::collections::HashSet",
        "std::sync::Mutex",
        "std::sync::RwLock",
        "std::fs::File",
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
        "std::any::Any",
        "std::sync::LazyLock",
        "std::sync::OnceLock",
        "once_cell::sync::Lazy",
        "once_cell::sync::OnceCell",
    ]
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}
