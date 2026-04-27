use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    severity: G3Severity,
    title: &'a str,
    message: &'a str,
    file: Option<&'a str>,
    line: Option<usize>,
    inventory: bool,
}

#[must_use]
pub fn findings<'a>(results: &'a [G3CheckResult], id: &str) -> Vec<Finding<'a>> {
    let mut findings = results
        .iter()
        .filter(|result| result.id() == id)
        .map(Finding::from_result)
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| {
        (
            format!("{:?}", left.severity),
            left.title,
            left.message,
            left.file,
            left.line,
            left.inventory,
        )
            .cmp(&(
                format!("{:?}", right.severity),
                right.title,
                right.message,
                right.file,
                right.line,
                right.inventory,
            ))
    });
    findings
}

#[must_use]
pub fn non_inventory(results: &[G3CheckResult]) -> Vec<Finding<'_>> {
    let mut findings = results
        .iter()
        .filter(|result| !result.inventory())
        .map(Finding::from_result)
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| {
        (
            format!("{:?}", left.severity),
            left.title,
            left.message,
            left.file,
            left.line,
            left.inventory,
        )
            .cmp(&(
                format!("{:?}", right.severity),
                right.title,
                right.message,
                right.file,
                right.line,
                right.inventory,
            ))
    });
    findings
}

#[must_use]
pub fn count(results: &[G3CheckResult], id: &str) -> usize {
    findings(results, id).len()
}

pub fn assert_present(
    results: &[G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
    inventory: bool,
) {
    let findings = findings(results, id);
    assert!(
        findings.iter().any(|finding| finding.title == title
            && finding.file == file
            && finding.inventory == inventory),
        "{findings:#?}"
    );
}

pub fn assert_message_contains(
    results: &[G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
    inventory: bool,
    needle: &str,
) {
    let findings = findings(results, id);
    assert!(
        findings.iter().any(|finding| {
            finding.title == title
                && finding.file == file
                && finding.inventory == inventory
                && finding.message.contains(needle)
        }),
        "{findings:#?}"
    );
}

pub fn assert_message_equals(
    results: &[G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
    inventory: bool,
    message: &str,
) {
    let findings = findings(results, id);
    assert!(
        findings.iter().any(|finding| {
            finding.title == title
                && finding.file == file
                && finding.inventory == inventory
                && finding.message == message
        }),
        "{findings:#?}"
    );
}

pub fn assert_present_on_line(
    results: &[G3CheckResult],
    id: &str,
    title: &str,
    file: Option<&str>,
    inventory: bool,
    line: usize,
) {
    let findings = findings(results, id);
    assert!(
        findings.iter().any(|finding| {
            finding.title == title
                && finding.file == file
                && finding.inventory == inventory
                && finding.line == Some(line)
        }),
        "{findings:#?}"
    );
}

pub fn assert_no_results_for_file(results: &[G3CheckResult], file: &str) {
    let findings = results
        .iter()
        .filter_map(|result| (result.file() == Some(file)).then(|| Finding::from_result(result)))
        .collect::<Vec<_>>();
    assert!(findings.is_empty(), "{findings:#?}");
}

impl<'a> Finding<'a> {
    fn from_result(result: &'a G3CheckResult) -> Self {
        Self {
            severity: result.severity(),
            title: result.title(),
            message: result.message(),
            file: result.file(),
            line: result.line(),
            inventory: result.inventory(),
        }
    }
}
