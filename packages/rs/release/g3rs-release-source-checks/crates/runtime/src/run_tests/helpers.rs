use g3rs_release_types::{
    G3RsReleaseInputFailure, G3RsReleaseSourceChecksInput, G3RsReleaseSourceReadme,
};

pub(super) fn input() -> G3RsReleaseSourceChecksInput {
    G3RsReleaseSourceChecksInput {
        readmes: vec![G3RsReleaseSourceReadme {
            crate_name: "demo".to_owned(),
            cargo_rel_path: "crates/demo/Cargo.toml".to_owned(),
            readme_rel_path: "crates/demo/README.md".to_owned(),
            content: "# Demo\n\nThis crate has a heading and enough content to satisfy the README quality rule. This text keeps going so the README is comfortably above the stub threshold for release checks.".to_owned(),
        }],
        input_failures: vec![G3RsReleaseInputFailure {
            rel_path: "crates/demo/README.md".to_owned(),
            message: "Failed to read README".to_owned(),
        }],
    }
}
