use std::collections::BTreeMap;

use g3rs_apparch_types::{
    G3RsApparchCrate, G3RsApparchLayer, G3RsApparchPublicItem, G3RsApparchPublicItemKind,
    G3RsApparchSourceChecksInput,
};
use guardrail3_check_types::G3CheckResult;

fn crate_input(layer: G3RsApparchLayer, cargo_rel_path: &str) -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: cargo_rel_path.replace('/', "-"),
        cargo_rel_path: cargo_rel_path.to_owned(),
        rel_dir: cargo_rel_path.trim_end_matches("/Cargo.toml").to_owned(),
        layer: Some(layer),
    }
}

pub(super) fn io_trait_input(
    layer: G3RsApparchLayer,
    cargo_rel_path: &str,
    rel_path: &str,
    item_name: &str,
) -> G3RsApparchSourceChecksInput {
    G3RsApparchSourceChecksInput {
        crates: vec![crate_input(layer, cargo_rel_path)],
        public_items: vec![G3RsApparchPublicItem {
            cargo_rel_path: cargo_rel_path.to_owned(),
            rel_path: rel_path.to_owned(),
            item_name: item_name.to_owned(),
            owner_name: None,
            kind: G3RsApparchPublicItemKind::Trait,
        }],
    }
}

pub(super) fn clean_io_input(
    layer: G3RsApparchLayer,
    cargo_rel_path: &str,
) -> G3RsApparchSourceChecksInput {
    G3RsApparchSourceChecksInput {
        crates: vec![crate_input(layer, cargo_rel_path)],
        public_items: Vec::new(),
    }
}

pub(super) fn logic_trait_input() -> G3RsApparchSourceChecksInput {
    G3RsApparchSourceChecksInput {
        crates: vec![crate_input(
            G3RsApparchLayer::Logic,
            "logic/service/Cargo.toml",
        )],
        public_items: vec![G3RsApparchPublicItem {
            cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
            rel_path: "logic/service/src/lib.rs".to_owned(),
            item_name: "ServiceRule".to_owned(),
            owner_name: None,
            kind: G3RsApparchPublicItemKind::Trait,
        }],
    }
}

pub(super) fn run_rule(input: &G3RsApparchSourceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let crates_by_path = input
        .crates
        .iter()
        .map(|krate| (krate.cargo_rel_path.clone(), krate))
        .collect::<BTreeMap<_, _>>();
    crate::rs_apparch_source_04_io_traits_in_types::check(input, &crates_by_path, &mut results);
    results
}
