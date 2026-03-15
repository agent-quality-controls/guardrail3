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

    pub fn error_count(&self) -> usize {
        self.sections
            .iter()
            .flat_map(|s| &s.results)
            .filter(|r| r.severity == Severity::Error)
            .count()
    }

    pub fn warn_count(&self) -> usize {
        self.sections
            .iter()
            .flat_map(|s| &s.results)
            .filter(|r| r.severity == Severity::Warn)
            .count()
    }

    pub fn info_count(&self) -> usize {
        self.sections
            .iter()
            .flat_map(|s| &s.results)
            .filter(|r| r.severity == Severity::Info)
            .count()
    }
}
