use guardrail3_domain_report::{CheckResult, Report};

pub fn assert_filtered_files(filtered: &[CheckResult], expected: &[&str]) {
    let files = filtered
        .iter()
        .map(|item| item.file.as_deref().unwrap_or("<none>"))
        .collect::<Vec<_>>();
    assert_eq!(files, expected);
}

pub fn assert_allowed(actual: bool) {
    assert!(actual);
}

pub fn assert_clean_section(report: &Report, section_name: &str) {
    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, section_name);
    let live_results = report.sections[0]
        .results
        .iter()
        .filter(|result| !result.inventory)
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
    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, section_name);
    let live_results = report.sections[0]
        .results
        .iter()
        .filter(|result| !result.inventory)
        .collect::<Vec<_>>();
    assert_eq!(live_results.len(), 1, "{report:#?}");
    assert_eq!(live_results[0].id, id);
    assert_eq!(live_results[0].file.as_deref(), file);
}

pub fn assert_live_ids_present(report: &Report, section_name: &str, expected_ids: &[&str]) {
    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, section_name);
    let ids = report.sections[0]
        .results
        .iter()
        .filter(|result| !result.inventory)
        .map(|result| result.id.as_str())
        .collect::<Vec<_>>();
    for expected in expected_ids {
        assert!(ids.contains(expected), "missing expected id `{expected}`: {report:#?}");
    }
}

pub fn assert_live_files_for_id(
    report: &Report,
    section_name: &str,
    id: &str,
    expected_files: &[&str],
) {
    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, section_name);
    let files = report.sections[0]
        .results
        .iter()
        .filter(|result| result.id == id && !result.inventory)
        .filter_map(|result| result.file.as_deref())
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
    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, section_name);
    assert!(
        report.sections[0].results.iter().any(|result| {
            result.id == id
                && file.is_none_or(|expected| result.file.as_deref() == Some(expected))
                && inventory.is_none_or(|expected| result.inventory == expected)
                && title.is_none_or(|expected| result.title == expected)
        }),
        "expected result not present: {report:#?}"
    );
}

pub fn assert_absent_file(report: &Report, section_name: &str, file: &str) {
    assert_eq!(report.sections.len(), 1, "unexpected sections: {report:#?}");
    assert_eq!(report.sections[0].name, section_name);
    assert!(
        !report.sections[0]
            .results
            .iter()
            .any(|result| result.file.as_deref() == Some(file)),
        "unexpected file `{file}` present in results: {report:#?}"
    );
}
