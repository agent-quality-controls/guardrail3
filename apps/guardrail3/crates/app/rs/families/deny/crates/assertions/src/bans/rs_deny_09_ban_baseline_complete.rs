crate::define_result_assertions!("RS-DENY-09");

pub fn assert_no_findings_for_file(
    results: &[guardrail3_domain_report::CheckResult],
    file: &str,
) {
    let matching = findings(results)
        .into_iter()
        .filter(|finding| finding.file == Some(file))
        .collect::<Vec<_>>();
    assert!(
        matching.is_empty(),
        "unexpected standalone app baseline findings for {file}: {matching:#?}"
    );
}
