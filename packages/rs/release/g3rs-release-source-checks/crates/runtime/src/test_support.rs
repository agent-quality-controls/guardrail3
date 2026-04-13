#![cfg(test)]

use g3rs_release_source_checks_types::{
    G3RsReleaseInputFailure, G3RsReleaseSourceChecksInput, G3RsReleaseSourceReadme,
};

pub(crate) fn source_input(content: &str) -> G3RsReleaseSourceChecksInput {
    G3RsReleaseSourceChecksInput {
        readmes: vec![G3RsReleaseSourceReadme {
            crate_name: "demo".to_owned(),
            cargo_rel_path: "crates/demo/Cargo.toml".to_owned(),
            readme_rel_path: "crates/demo/README.md".to_owned(),
            content: content.to_owned(),
        }],
        input_failures: Vec::new(),
    }
}

pub(crate) fn failure(rel_path: &str, message: &str) -> G3RsReleaseInputFailure {
    G3RsReleaseInputFailure {
        rel_path: rel_path.to_owned(),
        message: message.to_owned(),
    }
}
