#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warn,
    Info,
}

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub id: String,
    pub severity: Severity,
    pub title: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub results: Vec<CheckResult>,
}

#[derive(Debug)]
pub struct Report {
    pub project_path: String,
    pub stacks: Vec<String>,
    pub sections: Vec<Section>,
}

impl Report {
    pub const fn new(project_path: String, stacks: Vec<String>) -> Self {
        Self {
            project_path,
            stacks,
            sections: Vec::new(),
        }
    }

    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    fn count_by_severity(&self, severity: Severity) -> usize {
        self.sections
            .iter()
            .flat_map(|s| &s.results)
            .filter(|r| r.severity == severity)
            .count()
    }
}

/// Generate a severity-count method on `Report` to avoid structural duplication.
macro_rules! severity_counter {
    ($name:ident, $variant:ident) => {
        impl Report {
            pub fn $name(&self) -> usize {
                self.count_by_severity(Severity::$variant)
            }
        }
    };
}

severity_counter!(error_count, Error);
severity_counter!(warn_count, Warn);
severity_counter!(info_count, Info);

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Bug 3: Score formula / severity counting ----

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
                },
                CheckResult {
                    id: "R2".to_owned(),
                    severity: Severity::Warn,
                    title: "t".to_owned(),
                    message: "m".to_owned(),
                    file: None,
                    line: None,
                },
                CheckResult {
                    id: "R3".to_owned(),
                    severity: Severity::Info,
                    title: "t".to_owned(),
                    message: "m".to_owned(),
                    file: None,
                    line: None,
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
}
