use g3rs_release_filetree_checks_assertions::cliff_exists as assertions;
use g3rs_release_types::G3RsReleaseFileTreeRepo;

fn repo() -> G3RsReleaseFileTreeRepo {
    G3RsReleaseFileTreeRepo {
        cargo_rel_path: "Cargo.toml".to_owned(),
        publishable_count: 1,
        license_rel_path: Some("LICENSE".to_owned()),
        release_plz_rel_path: "release-plz.toml".to_owned(),
        release_plz_exists: true,
        cliff_rel_path: "cliff.toml".to_owned(),
        cliff_exists: true,
    }
}

#[test]
fn warns_when_cliff_is_missing() {
    let mut repo = repo();
    repo.cliff_exists = false;
    let mut results = Vec::new();
    super::super::check(&repo, &mut results);
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "cliff.toml missing",
            "Repo root is missing `cliff.toml`. Create `cliff.toml` at the repo root.",
            "cliff.toml",
            false,
        )],
    );
}

#[test]
fn skips_when_workspace_has_no_publishable_crates() {
    let mut repo = repo();
    repo.publishable_count = 0;
    repo.cliff_exists = false;
    let mut results = Vec::new();
    super::super::check(&repo, &mut results);
    assertions::assert_no_findings(&results);
}
