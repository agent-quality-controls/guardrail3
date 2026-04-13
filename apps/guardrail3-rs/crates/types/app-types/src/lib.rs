use std::path::{Path, PathBuf};

use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SupportedFamily {
    Topology,
    Toolchain,
    Fmt,
    Cargo,
    Clippy,
    Deny,
    Code,
    Arch,
    Deps,
    Garde,
    Test,
    Release,
    Hooks,
    Apparch,
}

impl SupportedFamily {
    pub const ALL: [Self; 14] = [
        Self::Topology,
        Self::Toolchain,
        Self::Fmt,
        Self::Cargo,
        Self::Clippy,
        Self::Deny,
        Self::Code,
        Self::Arch,
        Self::Deps,
        Self::Garde,
        Self::Test,
        Self::Release,
        Self::Hooks,
        Self::Apparch,
    ];

    #[must_use]
    pub const fn cli_name(self) -> &'static str {
        match self {
            Self::Topology => "topology",
            Self::Toolchain => "toolchain",
            Self::Fmt => "fmt",
            Self::Cargo => "cargo",
            Self::Clippy => "clippy",
            Self::Deny => "deny",
            Self::Code => "code",
            Self::Arch => "arch",
            Self::Deps => "deps",
            Self::Garde => "garde",
            Self::Test => "test",
            Self::Release => "release",
            Self::Hooks => "hooks",
            Self::Apparch => "apparch",
        }
    }

    #[must_use]
    pub fn parse_cli(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|family| family.cli_name() == value)
    }
}

#[derive(Debug, Clone)]
pub struct ValidateRequest {
    pub workspace_root: PathBuf,
    pub families: Vec<SupportedFamily>,
    pub include_inventory: bool,
}

impl ValidateRequest {
    #[must_use]
    pub fn selected_families(&self) -> Vec<SupportedFamily> {
        if self.families.is_empty() {
            return SupportedFamily::ALL.to_vec();
        }
        SupportedFamily::ALL
            .into_iter()
            .filter(|family| self.families.contains(family))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct FamilyRun {
    pub family: SupportedFamily,
    pub results: Vec<G3CheckResult>,
}

#[derive(Debug, Clone, Default)]
pub struct ValidateReport {
    pub runs: Vec<FamilyRun>,
}

impl ValidateReport {
    #[must_use]
    pub fn highest_severity(&self, include_inventory: bool) -> Option<G3Severity> {
        self.runs
            .iter()
            .flat_map(|run| run.results.iter())
            .filter(|result| include_inventory || !result.inventory())
            .map(G3CheckResult::severity)
            .max_by_key(|severity| match severity {
                G3Severity::Info => 0_u8,
                G3Severity::Warn => 1_u8,
                G3Severity::Error => 2_u8,
            })
    }
}

pub trait WorkspaceCrawler {
    fn crawl(&self, root: &Path) -> Result<G3RsWorkspaceCrawl, String>;
}

pub trait FamilyRunner {
    fn run_family(
        &self,
        family: SupportedFamily,
        crawl: &G3RsWorkspaceCrawl,
    ) -> Result<Vec<G3CheckResult>, String>;
}

pub trait ReportRenderer {
    fn render(&self, report: &ValidateReport, include_inventory: bool) -> String;
}

#[cfg(test)]
mod tests {
    use super::{SupportedFamily, ValidateRequest};

    #[test]
    fn selected_families_follow_canonical_order() {
        let request = ValidateRequest {
            workspace_root: "ignored".into(),
            families: vec![
                SupportedFamily::Release,
                SupportedFamily::Fmt,
                SupportedFamily::Toolchain,
            ],
            include_inventory: false,
        };

        assert_eq!(
            request.selected_families(),
            vec![
                SupportedFamily::Toolchain,
                SupportedFamily::Fmt,
                SupportedFamily::Release,
            ]
        );
    }
}
