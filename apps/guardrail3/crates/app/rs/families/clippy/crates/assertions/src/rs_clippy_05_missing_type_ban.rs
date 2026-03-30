use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-05";

pub fn managed_type_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::EXPECTED_TYPE_BANS.to_vec()
}

pub fn service_type_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::EXPECTED_TYPE_BANS.to_vec()
}

pub fn library_type_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::library_profile_type_paths()
}

pub fn assert_generated_service_type_bans(actual: &[&str]) {
    let expected = service_type_bans();
    assert_eq!(actual, expected);
}

pub fn assert_generated_library_type_bans_without_extras(actual: &[&str]) {
    let expected = service_type_bans();
    assert_eq!(actual, expected);
}

pub fn assert_golden(results: &[CheckResult], expected: &[&str], file: &str) {
    let expected_messages = expected
        .iter()
        .map(|path| format!("`{path}` is banned."))
        .collect::<Vec<_>>();
    let actual_messages = results
        .iter()
        .map(|result| result.message())
        .collect::<Vec<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id() == ID
            && result.inventory()
            && result.severity() == Severity::Info
            && result.title() == "type ban present"
            && result.file() == Some(file)
    }));
}

pub fn assert_service_type_bans(results: &[CheckResult], file: &str) {
    let expected = service_type_bans();
    assert_golden(results, &expected, file);
}

pub fn assert_malformed_section(results: &[CheckResult], file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.title() == "disallowed-types section malformed"
                && result.message() == "`disallowed-types` must be an array, found table."
                && !result.inventory()
                && result.file() == Some(file)
        }),
        "expected malformed section warning: {results:#?}"
    );
    assert!(results.iter().all(|result| !result.inventory()));
}

pub fn assert_garde_disabled(results: &[CheckResult], expected: &[&str], file: &str) {
    let expected_messages = expected
        .iter()
        .map(|path| format!("`{path}` is banned."))
        .collect::<Vec<_>>();
    let actual_messages = results
        .iter()
        .map(|result| result.message())
        .collect::<Vec<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id() == ID
            && result.inventory()
            && result.severity() == Severity::Info
            && result.title() == "type ban present"
            && result.file() == Some(file)
    }));
}

pub fn assert_excludes_library_global_state(results: &[CheckResult]) {
    assert!(results.iter().all(|result| result.id() == ID));
    assert!(!results.iter().any(|result| {
        result.message().contains("std::sync::LazyLock")
            || result.message().contains("std::sync::OnceLock")
            || result.message().contains("once_cell::sync::Lazy")
            || result.message().contains("once_cell::sync::OnceCell")
    }));
}

pub fn assert_missing_messages(results: &[CheckResult], expected: &[&str]) {
    let actual_errors = results
        .iter()
        .filter(|result| result.severity() == Severity::Error)
        .map(|result| result.message())
        .collect::<Vec<_>>();
    assert_eq!(actual_errors, expected);
    assert!(results.iter().all(|result| result.id() == ID));
}

pub fn expected_garde_disabled_type_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::EXPECTED_TYPE_BANS
        .iter()
        .filter(|path| {
            !matches!(
                **path,
                "axum::extract::Json"
                    | "axum::Json"
                    | "axum::extract::Query"
                    | "axum::extract::Form"
                    | "axum::extract::Path"
                    | "axum::extract::Multipart"
                    | "axum::extract::ConnectInfo"
                    | "axum_extra::extract::CookieJar"
                    | "axum_extra::extract::cookie::Cookie"
                    | "axum_extra::extract::TypedHeader"
                    | "axum_extra::extract::JsonDeserializer"
                    | "axum_extra::extract::JsonLines"
                    | "axum_extra::extract::Protobuf"
                    | "axum_extra::extract::Cbor"
                    | "axum_extra::extract::MsgPack"
            )
        })
        .copied()
        .collect()
}
