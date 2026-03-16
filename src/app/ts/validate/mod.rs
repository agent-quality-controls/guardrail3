pub mod ast_helpers;
pub mod config_files;
pub mod eslint_audit;
mod eslint_check;
mod jscpd_check;
mod npmrc_check;
mod package_check;
pub mod source_scan;
pub mod test_checks;
pub mod ts_code_analysis;
mod ts_comment_checks;
mod tsconfig_check;

use std::path::Path;

use crate::domain::report::ValidateDomains;
use crate::domain::report::{Report, Section};
use crate::ports::outbound::FileSystem;
pub fn run(
    fs: &dyn FileSystem,
    path: &Path,
    scoped_files: Option<&[String]>,
    domains: &ValidateDomains,
) -> Report {
    let mut report = Report::new(path.display().to_string(), vec!["TypeScript".to_owned()]);

    if domains.code {
        // Config file checks
        let config_results = config_files::check(fs, path);
        report.add_section(Section {
            name: "TS config files".to_owned(),
            results: config_results,
        });

        // Source code scan (respects scope flags)
        let source_results = source_scan::check(fs, path, scoped_files);
        report.add_section(Section {
            name: "TS source code scan".to_owned(),
            results: source_results,
        });
    }

    if domains.architecture {
        // ESLint boundary audit
        let eslint_results = eslint_audit::check(fs, path);
        report.add_section(Section {
            name: "ESLint boundary audit".to_owned(),
            results: eslint_results,
        });
    }

    if domains.tests {
        // Test quality checks
        let test_results = test_checks::check(fs, path);
        report.add_section(Section {
            name: "TS test quality".to_owned(),
            results: test_results,
        });
    }

    report
}
