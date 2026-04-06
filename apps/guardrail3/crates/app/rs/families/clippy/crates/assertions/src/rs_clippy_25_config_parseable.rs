use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-25";

pub fn service_clippy_toml() -> String {
    guardrail3_app_rs_family_clippy::clippy_support::build_clippy_toml(
        "service", false, true, "", "",
    )
}

pub fn assert_inventory(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id(), ID);
    assert!(result.inventory());
    assert_eq!(result.severity(), Severity::Info);
    assert_eq!(result.title(), "clippy.toml parseable");
    assert_eq!(result.message(), "`clippy.toml` parsed successfully.");
    assert_eq!(result.file(), Some(file));
}

pub fn assert_parse_error(results: &[CheckResult], file: &str, expected_fragment: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id(), ID);
    assert_eq!(result.severity(), Severity::Error);
    assert_eq!(result.title(), "clippy.toml parse error");
    assert!(
        result.message().contains(expected_fragment),
        "unexpected parse error message: {result:#?}"
    );
    assert_eq!(result.file(), Some(file));
    assert!(!result.inventory());
}

pub fn assert_missing_content(results: &[CheckResult], file: &str) {
    assert_parse_error(
        results,
        file,
        "clippy.toml content missing from ProjectTree",
    );
}

pub fn assert_single_owner(results: &[CheckResult], file: &str) {
    let parseability = results
        .iter()
        .filter(|result| result.id() == ID)
        .collect::<Vec<_>>();

    assert_eq!(parseability.len(), 1);
    let result = parseability[0];
    assert_eq!(result.severity(), Severity::Error);
    assert_eq!(result.file(), Some(file));

    let duplicate_parse_ids = [
        "RS-CLIPPY-CONFIG-01",
        "RS-CLIPPY-CONFIG-02",
        "RS-CLIPPY-04",
        "RS-CLIPPY-05",
        "RS-CLIPPY-06",
        "RS-CLIPPY-07",
        "RS-CLIPPY-08",
        "RS-CLIPPY-CONFIG-03",
        "RS-CLIPPY-CONFIG-04",
        "RS-CLIPPY-CONFIG-05",
        "RS-CLIPPY-13",
        "RS-CLIPPY-14",
        "RS-CLIPPY-15",
        "RS-CLIPPY-16",
        "RS-CLIPPY-CONFIG-06",
        "RS-CLIPPY-18",
        "RS-CLIPPY-19",
        "RS-CLIPPY-20",
        "RS-CLIPPY-CONFIG-07",
        "RS-CLIPPY-CONFIG-08",
    ];
    assert!(
        results
            .iter()
            .all(|result| { !(duplicate_parse_ids.contains(&result.id()) && !result.inventory()) }),
        "malformed clippy.toml must not fan out into dependent-rule errors: {results:#?}"
    );
}
