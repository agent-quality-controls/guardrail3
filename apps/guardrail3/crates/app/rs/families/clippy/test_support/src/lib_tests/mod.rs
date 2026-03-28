use std::collections::BTreeSet;

use super::build_fixture_clippy_toml;
use guardrail3_domain_modules::clippy::{
    EXPECTED_MACRO_BANS, LIBRARY_EXTRA_TYPE_PATHS, MAX_FN_PARAMS_BOOLS, MAX_STRUCT_BOOLS,
    SERVICE_METHOD_PATHS, THRESHOLD_VALUES, TOO_MANY_ARGUMENTS_THRESHOLD, TOO_MANY_LINES_THRESHOLD,
    TYPE_COMPLEXITY_THRESHOLD,
};

fn paths_for_key(parsed: &toml::Value, key: &str) -> BTreeSet<String> {
    parsed
        .get(key)
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| match entry {
            toml::Value::String(path) => Some(path.clone()),
            toml::Value::Table(table) => table
                .get("path")
                .and_then(toml::Value::as_str)
                .map(str::to_owned),
            _ => None,
        })
        .into_iter()
        .collect()
}

#[test]
fn generated_service_fixture_matches_checker_expectations() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", ""))
            .expect("valid clippy TOML");

    for (key, expected) in THRESHOLD_VALUES {
        assert_eq!(
            parsed.get(*key).and_then(toml::Value::as_integer),
            Some(*expected),
            "threshold drift for {}",
            key,
        );
    }

    assert_eq!(
        parsed
            .get("avoid-breaking-exported-api")
            .and_then(toml::Value::as_bool),
        Some(false)
    );
    assert_eq!(
        parsed
            .get("allow-dbg-in-tests")
            .and_then(toml::Value::as_bool),
        Some(false)
    );
    assert_eq!(
        parsed
            .get("allow-expect-in-tests")
            .and_then(toml::Value::as_bool),
        Some(true)
    );
    assert_eq!(
        parsed
            .get("allow-panic-in-tests")
            .and_then(toml::Value::as_bool),
        Some(false)
    );
    assert_eq!(
        parsed
            .get("allow-print-in-tests")
            .and_then(toml::Value::as_bool),
        Some(false)
    );
    assert_eq!(
        parsed
            .get("allow-unwrap-in-tests")
            .and_then(toml::Value::as_bool),
        Some(false)
    );

    let expected_methods = SERVICE_METHOD_PATHS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        paths_for_key(&parsed, "disallowed-methods"),
        expected_methods
    );

    let expected_types = [
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
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();
    assert_eq!(paths_for_key(&parsed, "disallowed-types"), expected_types);

    let expected_macros = EXPECTED_MACRO_BANS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(paths_for_key(&parsed, "disallowed-macros"), expected_macros);

    assert_eq!(
        parsed
            .get("max-struct-bools")
            .and_then(toml::Value::as_integer),
        Some(MAX_STRUCT_BOOLS),
    );
    assert_eq!(
        parsed
            .get("max-fn-params-bools")
            .and_then(toml::Value::as_integer),
        Some(MAX_FN_PARAMS_BOOLS),
    );
    assert_eq!(
        parsed
            .get("too-many-lines-threshold")
            .and_then(toml::Value::as_integer),
        Some(TOO_MANY_LINES_THRESHOLD),
    );
    assert_eq!(
        parsed
            .get("too-many-arguments-threshold")
            .and_then(toml::Value::as_integer),
        Some(TOO_MANY_ARGUMENTS_THRESHOLD),
    );
    assert_eq!(
        parsed
            .get("type-complexity-threshold")
            .and_then(toml::Value::as_integer),
        Some(TYPE_COMPLEXITY_THRESHOLD),
    );
    let library_only_types = LIBRARY_EXTRA_TYPE_PATHS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    assert!(library_only_types.is_disjoint(&paths_for_key(&parsed, "disallowed-types")));
}
