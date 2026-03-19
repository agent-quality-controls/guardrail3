use std::path::Path;

use crate::app::crawl::CrawlResult;
use crate::domain::report::ValidateDomains;
use crate::domain::report::{Report, Section};

use super::deploy_checks;
use super::hook_checks;
use crate::ports::outbound::{FileSystem, ToolChecker};

pub fn run(
    fs: &dyn FileSystem,
    path: &Path,
    has_rust: bool,
    has_typescript: bool,
    domains: &ValidateDomains,
    tc: &dyn ToolChecker,
    crawl: &CrawlResult,
) -> Report {
    let mut report = Report::new(path.display().to_string(), vec!["Hooks".to_owned()]);

    if domains.code {
        let mut hook_results = Vec::new();
        hook_checks::check_hooks(
            fs,
            tc,
            path,
            has_rust,
            has_typescript,
            &crawl.pre_commit_hooks,
            &mut hook_results,
        );
        report.add_section(Section {
            name: "Hook checks".to_owned(),
            results: hook_results,
        });

        // D1-D5 only run if the project has deployment configs
        let has_railpack = has_railpack_files(fs, path);
        let has_apps_dir = path.join("apps").is_dir();
        if has_railpack || has_apps_dir {
            let mut deploy_results = Vec::new();
            deploy_checks::check_deployment(fs, path, &mut deploy_results);
            report.add_section(Section {
                name: "Deployment checks".to_owned(),
                results: deploy_results,
            });
        }
    }

    report
}

fn has_railpack_files(fs: &dyn FileSystem, path: &Path) -> bool {
    let entries = fs.list_dir(path);
    for entry in entries {
        if let Some(name) = entry.file_name().to_str() {
            if name.starts_with("railpack-")
                && Path::new(name).extension().is_some_and(|e| e == "json")
            {
                return true;
            }
        }
    }
    false
}
