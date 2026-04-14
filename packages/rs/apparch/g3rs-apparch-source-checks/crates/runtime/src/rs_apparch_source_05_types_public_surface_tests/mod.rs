use g3rs_apparch_types::{
    G3RsApparchCrate, G3RsApparchLayer, G3RsApparchPublicItem, G3RsApparchPublicItemKind,
    G3RsApparchSourceChecksInput,
};
use guardrail3_check_types::G3Severity;

fn crate_input(layer: G3RsApparchLayer, cargo_rel_path: &str) -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: cargo_rel_path.replace('/', "-"),
        cargo_rel_path: cargo_rel_path.to_owned(),
        rel_dir: cargo_rel_path.trim_end_matches("/Cargo.toml").to_owned(),
        layer: Some(layer),
    }
}

#[test]
fn public_free_function_in_types_fires() {
    let input = G3RsApparchSourceChecksInput {
        crates: vec![crate_input(G3RsApparchLayer::Types, "types/contracts/Cargo.toml")],
        public_items: vec![G3RsApparchPublicItem {
            cargo_rel_path: "types/contracts/Cargo.toml".to_owned(),
            rel_path: "types/contracts/src/lib.rs".to_owned(),
            item_name: "choose_retry_strategy".to_owned(),
            owner_name: None,
            kind: G3RsApparchPublicItemKind::FreeFunction,
        }],
    };

    let results = crate::check(&input);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-SOURCE-05")
        .expect("types free-function violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("types/contracts/src/lib.rs"));
}

#[test]
fn public_inherent_method_in_types_fires() {
    let input = G3RsApparchSourceChecksInput {
        crates: vec![crate_input(G3RsApparchLayer::Types, "types/contracts/Cargo.toml")],
        public_items: vec![G3RsApparchPublicItem {
            cargo_rel_path: "types/contracts/Cargo.toml".to_owned(),
            rel_path: "types/contracts/src/order.rs".to_owned(),
            item_name: "save_to_db".to_owned(),
            owner_name: Some("OrderDto".to_owned()),
            kind: G3RsApparchPublicItemKind::InherentMethod,
        }],
    };

    let results = crate::check(&input);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-SOURCE-05")
        .expect("types inherent-method violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("types/contracts/src/order.rs"));
}

#[test]
fn trait_only_types_crate_emits_inventory() {
    let input = G3RsApparchSourceChecksInput {
        crates: vec![crate_input(G3RsApparchLayer::Types, "types/contracts/Cargo.toml")],
        public_items: vec![G3RsApparchPublicItem {
            cargo_rel_path: "types/contracts/Cargo.toml".to_owned(),
            rel_path: "types/contracts/src/lib.rs".to_owned(),
            item_name: "OutboundPort".to_owned(),
            owner_name: None,
            kind: G3RsApparchPublicItemKind::Trait,
        }],
    };

    let results = crate::check(&input);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-SOURCE-05")
        .expect("types inventory");

    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}
