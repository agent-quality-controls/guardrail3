#![cfg(test)]

use g3rs_release_filetree_checks_types::{
    G3RsReleaseFileTreeChecksInput, G3RsReleaseFileTreeReadme, G3RsReleaseFileTreeRepo,
    G3RsReleaseInputFailure,
};

pub(crate) fn repo_input() -> G3RsReleaseFileTreeChecksInput {
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

pub(crate) fn readme(crate_name: &str) -> G3RsReleaseFileTreeReadme {
    G3RsReleaseFileTreeReadme {
        crate_name: crate_name.to_owned(),
        cargo_rel_path: format!("crates/{crate_name}/Cargo.toml"),
        publishable: true,
        readme_declared_false: false,
        readme_rel_path: format!("crates/{crate_name}/README.md"),
        readme_exists: true,
    }
}

pub(crate) fn failure(rel_path: &str, message: &str) -> G3RsReleaseInputFailure {
    G3RsReleaseInputFailure {
        rel_path: rel_path.to_owned(),
        message: message.to_owned(),
    }
}
