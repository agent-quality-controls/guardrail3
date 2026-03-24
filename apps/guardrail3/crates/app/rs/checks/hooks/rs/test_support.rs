use std::collections::BTreeSet;
use std::path::Path;

use crate::app::rs::checks::hooks::shell::{ParsedShellScript, parse_script};
use crate::ports::outbound::{CommandRunResult, ToolChecker};

pub fn parsed_hook(content: &str) -> ParsedShellScript<'_> {
    parse_script(content)
}

pub struct StubToolChecker {
    installed: BTreeSet<&'static str>,
}

impl StubToolChecker {
    pub fn new(installed: &[&'static str]) -> Self {
        Self {
            installed: installed.iter().copied().collect(),
        }
    }
}

impl ToolChecker for StubToolChecker {
    fn is_installed(&self, tool: &str) -> bool {
        self.installed.contains(tool)
    }

    fn run_cargo_publish_dry_run_outcome(&self, _path: &Path) -> Option<CommandRunResult> {
        None
    }
}
