use guardrail3_domain_report::CheckResult;

const RULE_ID: &str = "RS-GARDE-05";

pub fn findings(results: &[CheckResult]) -> Vec<&CheckResult> {
    results
        .iter()
        .filter(|result| result.id == RULE_ID)
        .collect()
}

pub fn assert_rule_quiet(results: &[CheckResult]) {
    assert!(
        findings(results).is_empty(),
        "expected no {RULE_ID} findings"
    );
}

pub fn assert_rule_files(results: &[CheckResult], expected: &[&str]) {
    let mut files = findings(results)
        .into_iter()
        .filter_map(|result| result.file.clone())
        .collect::<Vec<_>>();
    files.sort();
    assert_eq!(
        files,
        expected
            .iter()
            .map(|file| (*file).to_owned())
            .collect::<Vec<_>>()
    );
}
