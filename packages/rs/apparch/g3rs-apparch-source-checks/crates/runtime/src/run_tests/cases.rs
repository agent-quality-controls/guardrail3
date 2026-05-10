use g3rs_apparch_source_checks_assertions::run as assertions;
use g3rs_apparch_types::{
    G3RsApparchCrate, G3RsApparchIoTraitsSourceChecksInput, G3RsApparchLayer,
    G3RsApparchPublicItem, G3RsApparchPublicItemKind, G3RsApparchSourceChecksInput,
    G3RsApparchTypesPublicSurfaceChecksInput,
};

#[test]
fn run_dispatches_prebound_source_inputs() {
    let io_crate = fixture_crate("io/outbound/db", G3RsApparchLayer::IoOutbound);
    let types_crate = fixture_crate("types/contracts", G3RsApparchLayer::Types);
    let input = G3RsApparchSourceChecksInput {
        io_traits_checks: vec![G3RsApparchIoTraitsSourceChecksInput {
            krate: io_crate.clone(),
            public_traits: vec![G3RsApparchPublicItem {
                cargo_rel_path: io_crate.cargo_rel_path,
                rel_path: "io/outbound/db/src/lib.rs".to_owned(),
                item_name: "DbPort".to_owned(),
                owner_name: None,
                kind: G3RsApparchPublicItemKind::Trait,
            }],
        }],
        types_public_surface_checks: vec![G3RsApparchTypesPublicSurfaceChecksInput {
            krate: types_crate.clone(),
            public_behavior_items: vec![G3RsApparchPublicItem {
                cargo_rel_path: types_crate.cargo_rel_path,
                rel_path: "types/contracts/src/lib.rs".to_owned(),
                item_name: "choose_retry_strategy".to_owned(),
                owner_name: None,
                kind: G3RsApparchPublicItemKind::FreeFunction,
            }],
        }],
    };

    let results = crate::run::check(&input);

    assertions::assert_has_finding_id(&results, "g3rs-apparch/io-traits-in-types");
    assertions::assert_has_finding_id(&results, "g3rs-apparch/types-public-surface");
}

fn fixture_crate(rel_dir: &str, layer: G3RsApparchLayer) -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: rel_dir
            .rsplit('/')
            .next()
            .expect("fixture crate path should end with a crate name")
            .to_owned(),
        cargo_rel_path: format!("{rel_dir}/Cargo.toml"),
        rel_dir: rel_dir.to_owned(),
        layer: Some(layer),
    }
}
