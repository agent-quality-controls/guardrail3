use crate::{CheckResult, Report, Section, Severity};

#[test]
fn error_count_only_counts_errors() {
    let mut report = Report::new("test".to_owned(), vec!["Rust".to_owned()]);
    report.add_section(Section::new(
        "test".to_owned(),
        vec![
            CheckResult::new(
                "R1".to_owned(),
                Severity::Error,
                "t".to_owned(),
                "m".to_owned(),
            ),
            CheckResult::new(
                "R2".to_owned(),
                Severity::Warn,
                "t".to_owned(),
                "m".to_owned(),
            ),
            CheckResult::new(
                "R3".to_owned(),
                Severity::Info,
                "t".to_owned(),
                "m".to_owned(),
            ),
        ],
    ));
    assert_eq!(report.error_count(), 1, "Should count only Error items");
    assert_eq!(report.warn_count(), 1, "Should count only Warn items");
    assert_eq!(report.info_count(), 1, "Should count only Info items");
}

#[test]
fn counts_across_multiple_sections() {
    let mut report = Report::new("test".to_owned(), vec!["Rust".to_owned()]);
    report.add_section(Section::new(
        "section1".to_owned(),
        vec![CheckResult::new(
            "R1".to_owned(),
            Severity::Error,
            "t".to_owned(),
            "m".to_owned(),
        )],
    ));
    report.add_section(Section::new(
        "section2".to_owned(),
        vec![CheckResult::new(
            "R2".to_owned(),
            Severity::Error,
            "t".to_owned(),
            "m".to_owned(),
        )],
    ));
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
