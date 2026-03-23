use std::collections::BTreeSet;

use super::super::super::dependency_facts::{Layer, PatchEntryFacts};
use super::super::super::inputs::PatchHexarchInput;
use super::super::check;

#[test]
fn only_layered_patch_and_replace_targets_error() {
    let patches = [
        PatchEntryFacts {
            cargo_rel_path: "apps/api/Cargo.toml".to_owned(),
            key: "api-domain".to_owned(),
            resolved_rel_dir: "apps/api/crates/domain".to_owned(),
            target_layer: Some(Layer::Domain),
        },
        PatchEntryFacts {
            cargo_rel_path: "apps/api/Cargo.toml".to_owned(),
            key: "api-ports".to_owned(),
            resolved_rel_dir: "apps/api/crates/ports".to_owned(),
            target_layer: Some(Layer::Ports),
        },
        PatchEntryFacts {
            cargo_rel_path: "apps/api/Cargo.toml".to_owned(),
            key: "shared-types".to_owned(),
            resolved_rel_dir: "packages/shared-types".to_owned(),
            target_layer: None,
        },
    ];

    let mut results = Vec::new();
    for patch in &patches {
        check(&PatchHexarchInput::new(patch), &mut results);
    }

    let actual_titles = results
        .iter()
        .map(|result| result.title.clone())
        .collect::<BTreeSet<_>>();
    let expected_titles = [
        "patch/replace entry `api-domain` bypasses hexarch dependency checks".to_owned(),
        "patch/replace entry `api-ports` bypasses hexarch dependency checks".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_titles, expected_titles,
        "unexpected patch hit set: {results:#?}"
    );
}
