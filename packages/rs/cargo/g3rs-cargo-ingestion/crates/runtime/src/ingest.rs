use cargo_toml_parser::types::CargoTomlDocument;
use g3rs_cargo_types::{
    G3RsCargoFileTreeRoot, G3RsCargoInputFailure, G3RsCargoMissingMember, G3RsCargoPolicyRoot,
    G3RsCargoPolicyRootKind, G3RsCargoRustPolicyState, G3RsCargoWorkspaceMember,
};

/// build root fn.
pub(crate) fn build_root(
    cargo_rel_path: String,
    cargo: CargoTomlDocument,
    rust_policy: G3RsCargoRustPolicyState,
) -> G3RsCargoPolicyRoot {
    let kind = match cargo_toml_parser::document::kind(&cargo) {
        cargo_toml_parser::types::CargoTomlDocumentKind::WorkspaceRoot => {
            G3RsCargoPolicyRootKind::WorkspaceRoot
        }
        cargo_toml_parser::types::CargoTomlDocumentKind::PackageRoot => {
            G3RsCargoPolicyRootKind::StandalonePackageRoot
        }
        cargo_toml_parser::types::CargoTomlDocumentKind::Other => G3RsCargoPolicyRootKind::Other,
    };

    G3RsCargoPolicyRoot {
        kind,
        rel_dir: String::new(),
        cargo_rel_path,
        cargo,
        rust_policy,
    }
}

/// fn const.
pub(crate) const fn build_member(
    member_rel: String,
    cargo_rel_path: String,
    cargo: CargoTomlDocument,
) -> G3RsCargoWorkspaceMember {
    G3RsCargoWorkspaceMember {
        workspace_root_rel: String::new(),
        member_rel,
        cargo_rel_path,
        cargo,
    }
}

/// filetree root fn.
pub(crate) fn filetree_root(
    kind: Option<G3RsCargoPolicyRootKind>,
    rust_policy_rel_path: Option<String>,
    members_parse_error: bool,
) -> G3RsCargoFileTreeRoot {
    G3RsCargoFileTreeRoot {
        kind,
        rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        rust_policy_rel_path,
        members_parse_error,
    }
}

/// input failure fn.
pub(crate) fn input_failure(
    rel_path: impl Into<String>,
    message: impl Into<String>,
) -> G3RsCargoInputFailure {
    G3RsCargoInputFailure {
        rel_path: rel_path.into(),
        message: message.into(),
    }
}

/// missing member fn.
pub(crate) fn missing_member(member_rel: String) -> G3RsCargoMissingMember {
    G3RsCargoMissingMember {
        workspace_root_rel: String::new(),
        workspace_cargo_rel_path: "Cargo.toml".to_owned(),
        member_rel,
    }
}
