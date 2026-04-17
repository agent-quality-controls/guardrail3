use g3rs_release_filetree_checks_assertions::rs_release_filetree_01_license_file as assertions;
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
fn inventories_when_license_exists() {
    let mut results = Vec::new();
    super::super::check(&repo(), &mut results);
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "LICENSE file exists",
            "Repo root includes `LICENSE`.",
            "LICENSE",
            true,
        )],
    );
}

#[test]
fn errors_when_license_is_missing() {
    let mut repo = repo();
    repo.license_rel_path = None;
    let mut results = Vec::new();
    super::super::check(&repo, &mut results);
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "LICENSE file missing",
            "Repo root is missing LICENSE material (`LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, or `LICENSE.md`). Create a LICENSE file at the repo root.",
            "Cargo.toml",
            false,
        )],
    );
}

#[test]
fn skips_when_workspace_has_no_publishable_crates() {
    let mut repo = repo();
    repo.publishable_count = 0;
    repo.license_rel_path = None;
    let mut results = Vec::new();
    super::super::check(&repo, &mut results);
    assertions::assert_no_findings(&results);
}
