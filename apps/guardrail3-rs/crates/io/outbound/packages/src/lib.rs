mod families;

use std::path::Path;

use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;
use guardrail3_rs_app_types::{FamilyRunner, SupportedFamily, WorkspaceCrawler};

#[derive(Debug, Default)]
pub struct PackageRuntime;

impl WorkspaceCrawler for PackageRuntime {
    fn crawl(&self, root: &Path) -> Result<G3RsWorkspaceCrawl, String> {
        g3rs_workspace_crawl::crawl(root).map_err(|error| format!("{error:?}"))
    }
}

impl FamilyRunner for PackageRuntime {
    fn run_family(
        &self,
        family: SupportedFamily,
        crawl: &G3RsWorkspaceCrawl,
    ) -> Result<Vec<G3CheckResult>, String> {
        match family {
            SupportedFamily::Topology => families::topology::run(crawl),
            SupportedFamily::Toolchain => families::toolchain::run(crawl),
            SupportedFamily::Fmt => families::fmt::run(crawl),
            SupportedFamily::Cargo => families::cargo::run(crawl),
            SupportedFamily::Clippy => families::clippy::run(crawl),
            SupportedFamily::Deny => families::deny::run(crawl),
            SupportedFamily::Code => families::code::run(crawl),
            SupportedFamily::Arch => families::arch::run(crawl),
            SupportedFamily::Deps => families::deps::run(crawl),
            SupportedFamily::Garde => families::garde::run(crawl),
            SupportedFamily::Test => families::test::run(crawl),
            SupportedFamily::Release => families::release::run(crawl),
            SupportedFamily::Hooks => families::hooks::run(crawl),
            SupportedFamily::Apparch => families::apparch::run(crawl),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use guardrail3_rs_app_types::{FamilyRunner, SupportedFamily, WorkspaceCrawler};

    use super::PackageRuntime;

    #[test]
    fn config_families_with_filetree_coverage_do_not_abort_when_root_config_is_missing() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(tempdir.path().join("src")).expect("create src dir");
        fs::write(
            tempdir.path().join("Cargo.toml"),
            "[workspace]\nmembers = []\n\n[package]\nname = \"smoke\"\nversion = \"0.1.0\"\nedition = \"2024\"\npublish = false\n",
        )
        .expect("write Cargo.toml");
        fs::write(tempdir.path().join("src/lib.rs"), "pub fn run() {}\n")
            .expect("write src/lib.rs");

        let runtime = PackageRuntime;
        let crawl = runtime.crawl(tempdir.path()).expect("crawl should succeed");

        let toolchain = runtime
            .run_family(SupportedFamily::Toolchain, &crawl)
            .expect("toolchain family should not abort on missing rust-toolchain.toml");
        assert!(toolchain.iter().any(|result| result.id() == "RS-TOOLCHAIN-FILETREE-01"));

        let fmt = runtime
            .run_family(SupportedFamily::Fmt, &crawl)
            .expect("fmt family should not abort on missing rustfmt.toml");
        assert!(fmt.iter().any(|result| result.id() == "RS-FMT-FILETREE-01"));

        let clippy = runtime
            .run_family(SupportedFamily::Clippy, &crawl)
            .expect("clippy family should not abort on missing clippy config");
        assert!(clippy.iter().any(|result| result.id() == "RS-CLIPPY-FILETREE-01"));

        let deny = runtime
            .run_family(SupportedFamily::Deny, &crawl)
            .expect("deny family should not abort on missing deny config");
        assert!(deny.iter().any(|result| result.id() == "RS-DENY-FILETREE-01"));
    }
}
