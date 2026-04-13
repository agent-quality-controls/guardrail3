use g3rs_cargo_types::{
    G3RsCargoFileTreeChecksInput, G3RsCargoFileTreeRoot, G3RsCargoInputFailure,
    G3RsCargoMissingMember, G3RsCargoPolicyRootKind,
};

#[test]
fn inventories_clean_workspace_filetree() {
    let input = G3RsCargoFileTreeChecksInput {
        root: G3RsCargoFileTreeRoot {
            kind: Some(G3RsCargoPolicyRootKind::WorkspaceRoot),
            rel_dir: String::new(),
            cargo_rel_path: "Cargo.toml".to_owned(),
            guardrail_rel_path: Some("guardrail3.toml".to_owned()),
            members_parse_error: false,
        },
        missing_members: Vec::new(),
        input_failures: Vec::new(),
    };

    let results = crate::run::check(&input);

    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-CARGO-FILETREE-10" && result.inventory()),
        "{results:#?}"
    );
    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-CARGO-FILETREE-14" && result.inventory()),
        "{results:#?}"
    );
}

#[test]
fn reports_missing_members_and_input_failures() {
    let input = G3RsCargoFileTreeChecksInput {
        root: G3RsCargoFileTreeRoot {
            kind: Some(G3RsCargoPolicyRootKind::WorkspaceRoot),
            rel_dir: String::new(),
            cargo_rel_path: "Cargo.toml".to_owned(),
            guardrail_rel_path: Some("guardrail3.toml".to_owned()),
            members_parse_error: false,
        },
        missing_members: vec![G3RsCargoMissingMember {
            workspace_root_rel: String::new(),
            workspace_cargo_rel_path: "Cargo.toml".to_owned(),
            member_rel: "crates/missing".to_owned(),
        }],
        input_failures: vec![G3RsCargoInputFailure {
            rel_path: "guardrail3.toml".to_owned(),
            message: "broken".to_owned(),
        }],
    };

    let results = crate::run::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CARGO-FILETREE-10"
                && result.title() == "declared workspace member missing Cargo.toml"
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CARGO-FILETREE-14"
                && result.title() == "failed to read Cargo configuration"
        }),
        "{results:#?}"
    );
}

#[test]
fn inventories_exactly_when_workspace_filetree_is_clean() {
    let input = G3RsCargoFileTreeChecksInput {
        root: G3RsCargoFileTreeRoot {
            kind: Some(G3RsCargoPolicyRootKind::WorkspaceRoot),
            rel_dir: String::new(),
            cargo_rel_path: "Cargo.toml".to_owned(),
            guardrail_rel_path: Some("guardrail3.toml".to_owned()),
            members_parse_error: false,
        },
        missing_members: Vec::new(),
        input_failures: Vec::new(),
    };

    let results = crate::run::check(&input);
    let ids: Vec<_> = results.iter().map(|result| result.id()).collect();

    assert_eq!(ids, vec!["RS-CARGO-FILETREE-10", "RS-CARGO-FILETREE-14"]);
    assert!(results.iter().all(|result| result.inventory()), "{results:#?}");
}
