use g3rs_apparch_types::{
    G3RsApparchCrate, G3RsApparchLayer, G3RsApparchPublicItem, G3RsApparchPublicItemKind,
    G3RsApparchTypesPublicSurfaceChecksInput,
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

pub(super) fn free_function_input() -> G3RsApparchTypesPublicSurfaceChecksInput {
    G3RsApparchTypesPublicSurfaceChecksInput {
        krate: crate_input(G3RsApparchLayer::Types, "types/contracts/Cargo.toml"),
        public_behavior_items: vec![G3RsApparchPublicItem {
            cargo_rel_path: "types/contracts/Cargo.toml".to_owned(),
            rel_path: "types/contracts/src/lib.rs".to_owned(),
            item_name: "choose_retry_strategy".to_owned(),
            owner_name: None,
            kind: G3RsApparchPublicItemKind::FreeFunction,
        }],
    }
}

pub(super) fn inherent_method_input() -> G3RsApparchTypesPublicSurfaceChecksInput {
    G3RsApparchTypesPublicSurfaceChecksInput {
        krate: crate_input(G3RsApparchLayer::Types, "types/contracts/Cargo.toml"),
        public_behavior_items: vec![G3RsApparchPublicItem {
            cargo_rel_path: "types/contracts/Cargo.toml".to_owned(),
            rel_path: "types/contracts/src/order.rs".to_owned(),
            item_name: "save_to_db".to_owned(),
            owner_name: Some("OrderDto".to_owned()),
            kind: G3RsApparchPublicItemKind::InherentMethod,
        }],
    }
}

pub(super) fn trait_only_input() -> G3RsApparchTypesPublicSurfaceChecksInput {
    G3RsApparchTypesPublicSurfaceChecksInput {
        krate: crate_input(G3RsApparchLayer::Types, "types/contracts/Cargo.toml"),
        public_behavior_items: Vec::new(),
    }
}

pub(super) fn run_rule(input: &G3RsApparchTypesPublicSurfaceChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::types_public_surface::check(input, &mut results);
    results
}
