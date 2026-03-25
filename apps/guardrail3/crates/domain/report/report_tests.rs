use crate::{CheckResult, Report, Section, Severity};

#[test]
fn error_count_only_counts_errors() {
    let mut report = Report::new("test".to_owned(), vec!["Rust".to_owned()]);
    report.add_section(Section {
        name: "test".to_owned(),
        results: vec![
            CheckResult {
                id: "R1".to_owned(),
                severity: Severity::Error,
                title: "t".to_owned(),
                message: "m".to_owned(),
                file: None,
                line: None,
                inventory: false,
            },
            CheckResult {
                id: "R2".to_owned(),
                severity: Severity::Warn,
                title: "t".to_owned(),
                message: "m".to_owned(),
                file: None,
                line: None,
                inventory: false,
            },
            CheckResult {
                id: "R3".to_owned(),
                severity: Severity::Info,
                title: "t".to_owned(),
                message: "m".to_owned(),
                file: None,
                line: None,
                inventory: false,
            },
        ],
    });
    assert_eq!(report.error_count(), 1, "Should count only Error items");
    assert_eq!(report.warn_count(), 1, "Should count only Warn items");
    assert_eq!(report.info_count(), 1, "Should count only Info items");
}

#[test]
fn counts_across_multiple_sections() {
    let mut report = Report::new("test".to_owned(), vec!["Rust".to_owned()]);
    report.add_section(Section {
        name: "section1".to_owned(),
        results: vec![CheckResult {
            id: "R1".to_owned(),
            severity: Severity::Error,
            title: "t".to_owned(),
            message: "m".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }],
    });
    report.add_section(Section {
        name: "section2".to_owned(),
        results: vec![CheckResult {
            id: "R2".to_owned(),
            severity: Severity::Error,
            title: "t".to_owned(),
            message: "m".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }],
    });
    assert_eq!(
        report.error_count(),
        2,
        "Should count errors across sections"
    );
}

#[test]
fn empty_report_has_zero_counts() {
    let report = Report::new("test".to_owned(), vec!["Rust".to_owned()]);
    assert_eq!(report.error_count(), 0);
    assert_eq!(report.warn_count(), 0);
    assert_eq!(report.info_count(), 0);
}
