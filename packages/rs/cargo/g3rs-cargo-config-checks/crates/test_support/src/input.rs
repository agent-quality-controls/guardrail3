use cargo_toml_parser::parse_document as parse_cargo_toml;
use g3rs_cargo_types::{
    G3RsCargoPolicyRoot, G3RsCargoPolicyRootKind, G3RsCargoRustPolicyState, G3RsCargoWaiver,
    G3RsCargoWorkspaceMember,
};
use guardrail3_rs_toml_parser::types::RustProfile;

/// Internal.
///
/// # Panics
///
/// See body for assertions.
#[must_use]
#[expect(
    clippy::expect_used,
    reason = "test-support fixture builder: malformed `cargo_toml` is a fixture bug, not a runtime error; surfacing it via panic gives a clear test failure"
)]
pub fn root(cargo_toml: &str, rust_policy: G3RsCargoRustPolicyState) -> G3RsCargoPolicyRoot {
    let cargo = parse_cargo_toml(cargo_toml).expect("cargo fixture should parse");
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
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
        rust_policy,
    }
}

/// Internal.
///
/// # Panics
///
/// See body for assertions.
#[must_use]
#[expect(
    clippy::expect_used,
    reason = "test-support fixture builder: malformed `cargo_toml` is a fixture bug, not a runtime error; surfacing it via panic gives a clear test failure"
)]
pub fn member(member_rel: &str, cargo_toml: &str) -> G3RsCargoWorkspaceMember {
    G3RsCargoWorkspaceMember {
        workspace_root_rel: String::new(),
        member_rel: member_rel.to_owned(),
        cargo_rel_path: format!("{member_rel}/Cargo.toml"),
        cargo: parse_cargo_toml(cargo_toml).expect("member cargo fixture should parse"),
    }
}

#[must_use]
pub fn waiver(rule: &str, file: &str, selector: &str, reason: &str) -> G3RsCargoWaiver {
    G3RsCargoWaiver {
        rule: rule.to_owned(),
        file: file.to_owned(),
        selector: selector.to_owned(),
        reason: reason.to_owned(),
    }
}

#[must_use]
pub fn parsed_rust_policy(
    profile: Option<RustProfile>,
    waivers: Vec<G3RsCargoWaiver>,
) -> G3RsCargoRustPolicyState {
    G3RsCargoRustPolicyState::Parsed {
        rel_path: "guardrail3-rs.toml".to_owned(),
        profile,
        waivers,
    }
}

#[must_use]
pub fn parse_error_rust_policy(reason: &str) -> G3RsCargoRustPolicyState {
    G3RsCargoRustPolicyState::ParseError {
        rel_path: "guardrail3-rs.toml".to_owned(),
        reason: reason.to_owned(),
    }
}
