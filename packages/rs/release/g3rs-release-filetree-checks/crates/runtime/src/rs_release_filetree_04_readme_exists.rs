use g3rs_release_filetree_checks_types::G3RsReleaseFileTreeReadme;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-RELEASE-FILETREE-04";

pub(crate) fn check(readme: &G3RsReleaseFileTreeReadme, results: &mut Vec<G3CheckResult>) {
    if !readme.publishable || readme.readme_declared_false {
        return;
    }

    if readme.readme_exists {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                format!("{}: README present", readme.crate_name),
                format!("README exists at `{}`.", readme.readme_rel_path),
                Some(readme.readme_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!("{}: README missing", readme.crate_name),
            format!(
                "Publishable crate `{}` is missing README content at `{}`. Create a README.md for this crate.",
                readme.crate_name, readme.readme_rel_path
            ),
            Some(readme.cargo_rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::test_support::readme;

    #[test]
    fn inventories_when_readme_exists() {
        let readme = readme("demo");
        let mut results = Vec::new();
        check(&readme, &mut results);
        assert_eq!(results[0].id(), "RS-RELEASE-FILETREE-04");
        assert_eq!(results[0].title(), "demo: README present");
        assert!(results[0].inventory());
    }

    #[test]
    fn errors_when_publishable_readme_is_missing() {
        let mut readme = readme("demo");
        readme.readme_exists = false;
        let mut results = Vec::new();

        check(&readme, &mut results);

        assert_eq!(results[0].id(), "RS-RELEASE-FILETREE-04");
        assert_eq!(results[0].title(), "demo: README missing");
        assert!(!results[0].inventory());
    }

    #[test]
    fn skips_non_publishable_and_readme_false_crates() {
        let mut non_publishable = readme("demo");
        non_publishable.publishable = false;
        let mut non_publishable_results = Vec::new();
        check(&non_publishable, &mut non_publishable_results);
        assert!(non_publishable_results.is_empty());

        let mut opted_out = readme("demo");
        opted_out.readme_declared_false = true;
        let mut opted_out_results = Vec::new();
        check(&opted_out, &mut opted_out_results);
        assert!(opted_out_results.is_empty());
    }
}
