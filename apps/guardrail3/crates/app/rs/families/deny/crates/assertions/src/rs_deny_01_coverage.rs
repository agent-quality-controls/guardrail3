crate::define_result_assertions!("RS-DENY-01");

pub fn assert_parse_error(results: &[guardrail3_domain_report::CheckResult], file: &str) {
    let parse_errors = findings(results)
        .into_iter()
        .filter(|finding| finding.title == "deny config parse error")
        .collect::<Vec<_>>();
    assert_eq!(
        parse_errors.len(),
        1,
        "unexpected RS-DENY-01 parse errors: {parse_errors:#?}"
    );
    let finding = &parse_errors[0];
    assert_eq!(finding.severity, guardrail3_domain_report::Severity::Error);
    assert_eq!(finding.file, Some(file));
    assert!(!finding.inventory);
    assert!(
        finding
            .message
            .starts_with(&format!("`{file}` could not be parsed: "))
    );
}
