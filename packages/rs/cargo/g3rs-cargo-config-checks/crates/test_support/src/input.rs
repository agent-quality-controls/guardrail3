use cargo_toml_parser::parse as parse_cargo_toml;
use g3rs_cargo_types::{
    G3RsCargoPolicyRoot, G3RsCargoPolicyRootKind, G3RsCargoRustPolicyState, G3RsCargoWaiver,
    G3RsCargoWorkspaceMember,
};
use guardrail3_rs_toml_parser::RustProfile;

pub fn root(cargo_toml: &str, rust_policy: G3RsCargoRustPolicyState) -> G3RsCargoPolicyRoot {
    let cargo = parse_cargo_toml(cargo_toml).expect("cargo fixture should parse");
    let raw_cargo = toml::from_str::<toml::Value>(cargo_toml).expect("raw cargo fixture should parse");
    let kind = if raw_cargo.get("workspace").is_some() {
        G3RsCargoPolicyRootKind::WorkspaceRoot
    } else if raw_cargo.get("package").is_some() {
        G3RsCargoPolicyRootKind::StandalonePackageRoot
    } else {
        G3RsCargoPolicyRootKind::Other
    };

    G3RsCargoPolicyRoot {
        kind,
        rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo,
        raw_cargo: raw_cargo.clone(),
        rust_policy,
        edition: root_field(&raw_cargo, kind, "edition"),
        edition_invalid: root_field_invalid(&raw_cargo, kind, "edition"),
        rust_version: root_field(&raw_cargo, kind, "rust-version"),
        rust_version_invalid: root_field_invalid(&raw_cargo, kind, "rust-version"),
    }
}

pub fn member(member_rel: &str, cargo_toml: &str) -> G3RsCargoWorkspaceMember {
    let raw_cargo = toml::from_str::<toml::Value>(cargo_toml).expect("member cargo fixture should parse");
    G3RsCargoWorkspaceMember {
        workspace_root_rel: String::new(),
        member_rel: member_rel.to_owned(),
        cargo_rel_path: format!("{member_rel}/Cargo.toml"),
        raw_cargo: raw_cargo.clone(),
        package_name: raw_cargo
            .get("package")
            .and_then(|value| value.get("name"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned),
        edition: raw_cargo
            .get("package")
            .and_then(|value| value.get("edition"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned),
        edition_invalid: raw_cargo
            .get("package")
            .and_then(|value| value.get("edition"))
            .is_some_and(|value| value.as_str().is_none()),
        lint_workspace_invalid: raw_cargo
            .get("lints")
            .and_then(|value| value.get("workspace"))
            .is_some_and(|value| value.as_bool().is_none()),
        lint_workspace_true: raw_cargo
            .get("lints")
            .and_then(|value| value.get("workspace"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false),
    }
}

pub fn waiver(rule: &str, file: &str, selector: &str, reason: &str) -> G3RsCargoWaiver {
    G3RsCargoWaiver {
        rule: rule.to_owned(),
        file: file.to_owned(),
        selector: selector.to_owned(),
        reason: reason.to_owned(),
    }
}

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

pub fn parse_error_rust_policy(reason: &str) -> G3RsCargoRustPolicyState {
    G3RsCargoRustPolicyState::ParseError {
        rel_path: "guardrail3-rs.toml".to_owned(),
        reason: reason.to_owned(),
    }
}

fn root_field(raw_cargo: &toml::Value, kind: G3RsCargoPolicyRootKind, field: &str) -> Option<String> {
    if kind == G3RsCargoPolicyRootKind::WorkspaceRoot {
        raw_cargo
            .get("workspace")
            .and_then(|value| value.get("package"))
            .and_then(|value| value.get(field))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
            .or_else(|| {
                raw_cargo
                    .get("package")
                    .and_then(|value| value.get(field))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
            })
    } else {
        raw_cargo
            .get("package")
            .and_then(|value| value.get(field))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
    }
}

fn root_field_invalid(raw_cargo: &toml::Value, kind: G3RsCargoPolicyRootKind, field: &str) -> bool {
    if kind == G3RsCargoPolicyRootKind::WorkspaceRoot {
        let workspace_value = raw_cargo
            .get("workspace")
            .and_then(|value| value.get("package"))
            .and_then(|value| value.get(field));
        if let Some(value) = workspace_value {
            return value.as_str().is_none();
        }
    }

    raw_cargo
        .get("package")
        .and_then(|value| value.get(field))
        .is_some_and(|value| value.as_str().is_none())
}
