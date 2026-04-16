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

pub(super) fn free_function_input() -> G3RsApparchSourceChecksInput {
    G3RsApparchSourceChecksInput {
        crates: vec![crate_input(G3RsApparchLayer::Types, "types/contracts/Cargo.toml")],
        public_items: vec![G3RsApparchPublicItem {
            cargo_rel_path: "types/contracts/Cargo.toml".to_owned(),
            rel_path: "types/contracts/src/lib.rs".to_owned(),
            item_name: "choose_retry_strategy".to_owned(),
            owner_name: None,
            kind: G3RsApparchPublicItemKind::FreeFunction,
        }],
    }
}

pub(super) fn inherent_method_input() -> G3RsApparchSourceChecksInput {
    G3RsApparchSourceChecksInput {
        crates: vec![crate_input(G3RsApparchLayer::Types, "types/contracts/Cargo.toml")],
        public_items: vec![G3RsApparchPublicItem {
            cargo_rel_path: "types/contracts/Cargo.toml".to_owned(),
            rel_path: "types/contracts/src/order.rs".to_owned(),
            item_name: "save_to_db".to_owned(),
            owner_name: Some("OrderDto".to_owned()),
            kind: G3RsApparchPublicItemKind::InherentMethod,
        }],
    }
}

pub(super) fn trait_only_input() -> G3RsApparchSourceChecksInput {
    G3RsApparchSourceChecksInput {
        crates: vec![crate_input(G3RsApparchLayer::Types, "types/contracts/Cargo.toml")],
        public_items: vec![G3RsApparchPublicItem {
            cargo_rel_path: "types/contracts/Cargo.toml".to_owned(),
            rel_path: "types/contracts/src/lib.rs".to_owned(),
            item_name: "OutboundPort".to_owned(),
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
    crate::rs_apparch_source_05_types_public_surface::check(input, &crates_by_path, &mut results);
    results
}
