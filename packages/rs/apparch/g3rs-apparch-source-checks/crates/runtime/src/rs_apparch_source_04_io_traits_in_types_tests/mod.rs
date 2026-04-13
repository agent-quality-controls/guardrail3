use g3rs_apparch_types::{
    G3RsApparchCrate, G3RsApparchLayer, G3RsApparchPublicTrait, G3RsApparchSourceChecksInput,
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
fn io_public_trait_fires() {
    let input = G3RsApparchSourceChecksInput {
        crates: vec![crate_input(G3RsApparchLayer::IoOutbound, "io/outbound/db/Cargo.toml")],
        public_traits: vec![G3RsApparchPublicTrait {
            cargo_rel_path: "io/outbound/db/Cargo.toml".to_owned(),
            rel_path: "io/outbound/db/src/lib.rs".to_owned(),
            trait_name: "DbPort".to_owned(),
        }],
    };

    let results = crate::check(&input);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-SOURCE-04")
        .expect("source violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("io/outbound/db/src/lib.rs"));
}

#[test]
fn io_inbound_public_trait_fires() {
    let input = G3RsApparchSourceChecksInput {
        crates: vec![crate_input(G3RsApparchLayer::IoInbound, "io/inbound/http/Cargo.toml")],
        public_traits: vec![G3RsApparchPublicTrait {
            cargo_rel_path: "io/inbound/http/Cargo.toml".to_owned(),
            rel_path: "io/inbound/http/src/lib.rs".to_owned(),
            trait_name: "InboundPort".to_owned(),
        }],
    };

    let results = crate::check(&input);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-SOURCE-04")
        .expect("source inbound violation");

    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.file(), Some("io/inbound/http/src/lib.rs"));
}

#[test]
fn logic_public_trait_stays_quiet() {
    let input = G3RsApparchSourceChecksInput {
        crates: vec![crate_input(G3RsApparchLayer::Logic, "logic/service/Cargo.toml")],
        public_traits: vec![G3RsApparchPublicTrait {
            cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
            rel_path: "logic/service/src/lib.rs".to_owned(),
            trait_name: "ServiceRule".to_owned(),
        }],
    };

    let results = crate::check(&input);
    assert!(results.is_empty());
}

#[test]
fn clean_io_crate_emits_inventory() {
    let input = G3RsApparchSourceChecksInput {
        crates: vec![crate_input(G3RsApparchLayer::IoInbound, "io/inbound/http/Cargo.toml")],
        public_traits: Vec::new(),
    };

    let results = crate::check(&input);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-SOURCE-04")
        .expect("source inventory");

    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}

#[test]
fn clean_outbound_io_crate_emits_inventory() {
    let input = G3RsApparchSourceChecksInput {
        crates: vec![crate_input(G3RsApparchLayer::IoOutbound, "io/outbound/db/Cargo.toml")],
        public_traits: Vec::new(),
    };

    let results = crate::check(&input);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-SOURCE-04")
        .expect("outbound source inventory");

    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}
