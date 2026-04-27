use g3rs_release_filetree_checks_assertions::readme_exists as assertions;
use g3rs_release_types::G3RsReleaseFileTreeReadme;

fn readme() -> G3RsReleaseFileTreeReadme {
    G3RsReleaseFileTreeReadme {
        crate_name: "demo".to_owned(),
        cargo_rel_path: "crates/demo/Cargo.toml".to_owned(),
        publishable: true,
        readme_declared_false: false,
        readme_rel_path: "crates/demo/README.md".to_owned(),
        readme_exists: true,
    }
}

#[test]
fn inventories_when_readme_exists() {
    let mut results = Vec::new();
    super::super::check(&readme(), &mut results);
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "demo: README present",
            "README exists at `crates/demo/README.md`.",
            "crates/demo/README.md",
            true,
        )],
    );
}

#[test]
fn errors_when_publishable_readme_is_missing() {
    let mut readme = readme();
    readme.readme_exists = false;
    let mut results = Vec::new();
    super::super::check(&readme, &mut results);
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "demo: README missing",
            "Publishable crate `demo` is missing README content at `crates/demo/README.md`. Create a README.md for this crate.",
            "crates/demo/Cargo.toml",
            false,
        )],
    );
}

#[test]
fn skips_non_publishable_and_readme_false_crates() {
    let mut non_publishable = readme();
    non_publishable.publishable = false;
    let mut non_publishable_results = Vec::new();
    super::super::check(&non_publishable, &mut non_publishable_results);
    assertions::assert_no_findings(&non_publishable_results);

    let mut opted_out = readme();
    opted_out.readme_declared_false = true;
    let mut opted_out_results = Vec::new();
    super::super::check(&opted_out, &mut opted_out_results);
    assertions::assert_no_findings(&opted_out_results);
}
