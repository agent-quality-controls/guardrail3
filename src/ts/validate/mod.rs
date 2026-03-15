pub mod config_files;
pub mod eslint_audit;
pub mod source_scan;

use std::path::Path;

use crate::report::types::{Report, Section};

pub fn run(path: &Path, scoped_files: Option<&[String]>) -> Report {
    let mut report = Report::new(
        path.display().to_string(),
        vec!["TypeScript".to_string()],
    );

    // Config file checks (always run fully)
    let config_results = config_files::check(path);
    report.add_section(Section {
        name: "TS config files".to_string(),
        results: config_results,
    });

    // Source code scan (respects scope flags)
    let source_results = source_scan::check(path, scoped_files);
    report.add_section(Section {
        name: "TS source code scan".to_string(),
        results: source_results,
    });

    // ESLint boundary audit
    let eslint_results = eslint_audit::check(path);
    report.add_section(Section {
        name: "ESLint boundary audit".to_string(),
        results: eslint_results,
    });

    report
}
