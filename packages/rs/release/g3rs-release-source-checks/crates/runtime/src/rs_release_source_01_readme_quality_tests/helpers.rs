use g3rs_release_types::G3RsReleaseSourceReadme;
use guardrail3_check_types::G3CheckResult;

pub(super) fn input(content: &str) -> G3RsReleaseSourceReadme {
    G3RsReleaseSourceReadme {
        crate_name: "demo".to_owned(),
        cargo_rel_path: "crates/demo/Cargo.toml".to_owned(),
        readme_rel_path: "crates/demo/README.md".to_owned(),
        content: content.to_owned(),
    }
}

pub(super) fn check(content: &str) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    super::super::check(&input(content), &mut results);
    results
}
