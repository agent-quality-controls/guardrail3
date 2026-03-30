use guardrail3_domain_report::{CheckResult, Report};

pub fn assert_filtered_files(filtered: &[CheckResult], expected: &[&str]) {
    let files = filtered
        .iter()
        .map(|item| item.file().unwrap_or("<none>"))
        .collect::<Vec<_>>();
    assert_eq!(files, expected);
}

pub fn assert_allowed(actual: bool) {
    assert!(actual);
}

pub fn assert_clean_section(report: &Report, section_name: &str) {
    assert_eq!(
        report.sections().len(),
        1,
        "unexpected sections: {report:#?}"
    );
    assert_eq!(report.sections()[0].name(), section_name);
    let live_results = report.sections()[0]
        .results()
        .iter()
        .filter(|result| !result.inventory())
        .collect::<Vec<_>>();
    assert!(
        live_results.is_empty(),
        "clean section should not emit live findings: {report:#?}"
    );
}

pub fn assert_single_live_result(
    report: &Report,
    section_name: &str,
    id: &str,
    file: Option<&str>,
) {
    assert_eq!(
        report.sections().len(),
        1,
        "unexpected sections: {report:#?}"
    );
    assert_eq!(report.sections()[0].name(), section_name);
    let live_results = report.sections()[0]
        .results()
        .iter()
        .filter(|result| !result.inventory())
        .collect::<Vec<_>>();
    assert_eq!(live_results.len(), 1, "{report:#?}");
    assert_eq!(live_results[0].id(), id);
    assert_eq!(live_results[0].file(), file);
}

pub fn assert_live_ids_present(report: &Report, section_name: &str, expected_ids: &[&str]) {
    assert_eq!(
        report.sections().len(),
        1,
        "unexpected sections: {report:#?}"
    );
    assert_eq!(report.sections()[0].name(), section_name);
    let ids = report.sections()[0]
        .results()
        .iter()
        .filter(|result| !result.inventory())
        .map(CheckResult::id)
        .collect::<Vec<_>>();
    for expected in expected_ids {
        assert!(
            ids.contains(expected),
            "missing expected id `{expected}`: {report:#?}"
        );
    }
}

pub fn assert_live_files_for_id(
    report: &Report,
    section_name: &str,
    id: &str,
    expected_files: &[&str],
) {
    assert_eq!(
        report.sections().len(),
        1,
        "unexpected sections: {report:#?}"
    );
    assert_eq!(report.sections()[0].name(), section_name);
    let files = report.sections()[0]
        .results()
        .iter()
        .filter(|result| result.id() == id && !result.inventory())
        .filter_map(CheckResult::file)
        .collect::<Vec<_>>();
    assert_eq!(files, expected_files, "{report:#?}");
}

pub fn assert_result_present(
    report: &Report,
    section_name: &str,
    id: &str,
    file: Option<&str>,
    inventory: Option<bool>,
    title: Option<&str>,
) {
    assert_eq!(
        report.sections().len(),
        1,
        "unexpected sections: {report:#?}"
    );
    assert_eq!(report.sections()[0].name(), section_name);
    assert!(
        report.sections()[0].results().iter().any(|result| {
            result.id() == id
                && file.is_none_or(|expected| result.file() == Some(expected))
                && inventory.is_none_or(|expected| result.inventory() == expected)
                && title.is_none_or(|expected| result.title() == expected)
        }),
        "expected result not present: {report:#?}"
    );
}

pub fn assert_absent_file(report: &Report, section_name: &str, file: &str) {
    assert_eq!(
        report.sections().len(),
        1,
        "unexpected sections: {report:#?}"
    );
    assert_eq!(report.sections()[0].name(), section_name);
    assert!(
        !report.sections()[0]
            .results()
            .iter()
            .any(|result| result.file() == Some(file)),
        "unexpected file `{file}` present in results: {report:#?}"
    );
}

pub fn assert_arch_scoped_config_violation(report: &Report) {
    assert_single_live_result(report, "arch", "RS-ARCH-05", Some("guardrail3.toml"));
}

pub fn assert_arch_fail_closed_malformed_config(report: &Report) {
    assert_live_ids_present(report, "arch", &["RS-ARCH-02", "RS-ARCH-07"]);
}

pub fn assert_arch_fail_closed_malformed_governed_manifest(report: &Report) {
    assert_result_present(
        report,
        "arch",
        "RS-ARCH-07",
        Some("apps/backend/Cargo.toml"),
        None,
        None,
    );
}

pub fn assert_arch_app_scoped_hexarch_override(report: &Report) {
    assert_live_files_for_id(
        report,
        "arch",
        "RS-ARCH-06",
        &[
            "apps/backend/Cargo.toml",
            "apps/backend/crates/worker/Cargo.toml",
        ],
    );
}

pub fn assert_arch_inactive_misplaced_root_inventory(report: &Report) {
    assert_result_present(
        report,
        "arch",
        "RS-ARCH-02",
        None,
        Some(true),
        Some("Misplaced-root reporting is inactive"),
    );
}

pub fn assert_hexarch_fail_closed_malformed_config(report: &Report) {
    assert_live_ids_present(report, "hexarch", &["RS-HEXARCH-15"]);
}

pub fn assert_code_fail_closed_malformed_config(report: &Report) {
    assert_live_ids_present(report, "code", &["RS-CODE-30"]);
}

pub fn assert_code_scoped_files_config_result(report: &Report) {
    assert_result_present(
        report,
        "code",
        "RS-CODE-07",
        Some("apps/backend/rustfmt.toml"),
        None,
        None,
    );
}

pub fn assert_toolchain_requires_local_workspace_toolchain(report: &Report) {
    assert_live_ids_present(report, "toolchain", &["RS-TOOLCHAIN-01", "RS-TOOLCHAIN-02"]);
    assert_absent_file(report, "toolchain", "rust-toolchain.toml");
    assert_result_present(
        report,
        "toolchain",
        "RS-TOOLCHAIN-01",
        Some("apps/guardrail3/rust-toolchain.toml"),
        None,
        None,
    );
}
