use g3rs_release_filetree_checks_assertions::run as assertions;
use g3rs_release_types::{
    G3RsReleaseFileTreeChecksInput, G3RsReleaseFileTreeReadme, G3RsReleaseFileTreeRepo,
    G3RsReleaseInputFailure,
};

fn input() -> G3RsReleaseFileTreeChecksInput {
    G3RsReleaseFileTreeChecksInput {
        repo: Some(G3RsReleaseFileTreeRepo {
            cargo_rel_path: "Cargo.toml".to_owned(),
            publishable_count: 1,
            license_rel_path: Some("LICENSE".to_owned()),
            release_plz_rel_path: "release-plz.toml".to_owned(),
            release_plz_exists: true,
            cliff_rel_path: "cliff.toml".to_owned(),
            cliff_exists: true,
        }),
        readmes: Vec::new(),
        input_failures: Vec::new(),
    }
}

#[test]
fn aggregates_repo_readme_and_input_failures() {
    let mut input = input();
    input.readmes.push(G3RsReleaseFileTreeReadme {
        crate_name: "demo".to_owned(),
        cargo_rel_path: "crates/demo/Cargo.toml".to_owned(),
        publishable: true,
        readme_declared_false: false,
        readme_rel_path: "crates/demo/README.md".to_owned(),
        readme_exists: true,
    });
    input.input_failures.push(G3RsReleaseInputFailure {
        rel_path: "crates/demo/README.md".to_owned(),
        message: "Failed to read README for release checks.".to_owned(),
    });

    let results = super::super::check(&input);

    assertions::assert_result_ids(
        &results,
        &[
            "g3rs-release/filetree-input-failures",
            "g3rs-release/license-file",
            "g3rs-release/release-plz-exists",
            "g3rs-release/cliff-exists",
            "g3rs-release/readme-exists",
        ],
    );
}

#[test]
fn skips_workspace_release_files_when_nothing_publishes() {
    let mut input = input();
    let repo = input.repo.as_mut().expect("repo should exist");
    repo.publishable_count = 0;
    repo.license_rel_path = None;
    repo.release_plz_exists = false;
    repo.cliff_exists = false;

    let results = super::super::check(&input);

    assertions::assert_no_findings(&results);
}
