pub mod config_files;
pub mod eslint_audit;
mod eslint_check;
mod jscpd_check;
mod npmrc_check;
mod package_check;
pub mod source_scan;
mod tsconfig_check;

use std::path::Path;

use crate::report::types::{Report, Section};

pub fn run(path: &Path, scoped_files: Option<&[String]>) -> Report {
    let mut report = Report::new(path.display().to_string(), vec!["TypeScript".to_owned()]);

    // Config file checks (always run fully)
    let config_results = config_files::check(path);
    report.add_section(Section {
        name: "TS config files".to_owned(),
        results: config_results,
    });

    // Source code scan (respects scope flags)
    let source_results = source_scan::check(path, scoped_files);
    report.add_section(Section {
        name: "TS source code scan".to_owned(),
        results: source_results,
    });

    // ESLint boundary audit
    let eslint_results = eslint_audit::check(path);
    report.add_section(Section {
        name: "ESLint boundary audit".to_owned(),
        results: eslint_results,
    });

    report
}
