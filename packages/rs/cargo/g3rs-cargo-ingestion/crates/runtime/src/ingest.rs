use cargo_toml_parser::CargoToml;
use g3rs_cargo_types::{
    G3RsCargoEscapeHatch, G3RsCargoFileTreeRoot, G3RsCargoInputFailure,
    G3RsCargoMissingMember, G3RsCargoPolicyRoot, G3RsCargoPolicyRootKind,
    G3RsCargoWorkspaceMember,
};

pub(crate) fn build_root(
    cargo_rel_path: String,
    cargo: CargoToml,
    raw_cargo: toml::Value,
    guardrail_rel_path: Option<String>,
    guardrail_state: &GuardrailState,
) -> G3RsCargoPolicyRoot {
    let kind = crate::select::workspace_root_kind(&raw_cargo);
    let edition = root_package_field(&raw_cargo, kind, "edition");
    let rust_version = root_package_field(&raw_cargo, kind, "rust-version");

    G3RsCargoPolicyRoot {
        kind,
        rel_dir: String::new(),
        cargo_rel_path,
        cargo,
        raw_cargo,
        guardrail_rel_path,
        profile_name: guardrail_state.profile_name.clone(),
        escape_hatches: guardrail_state.escape_hatches.clone(),
        guardrail_parse_error: guardrail_state.parse_error,
        edition: edition.value,
        edition_invalid: edition.invalid,
        rust_version: rust_version.value,
        rust_version_invalid: rust_version.invalid,
    }
}

pub(crate) fn build_member(
    member_rel: String,
    cargo_rel_path: String,
    raw_cargo: toml::Value,
) -> G3RsCargoWorkspaceMember {
    G3RsCargoWorkspaceMember {
        workspace_root_rel: String::new(),
        member_rel,
        cargo_rel_path,
        package_name: raw_cargo
            .get("package")
            .and_then(|value| value.get("name"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned),
        edition: package_field(&raw_cargo, "edition").value,
        edition_invalid: package_field(&raw_cargo, "edition").invalid,
        lint_workspace_invalid: raw_cargo
            .get("lints")
            .and_then(|value| value.get("workspace"))
            .is_some_and(|value| value.as_bool().is_none()),
        lint_workspace_true: raw_cargo
            .get("lints")
            .and_then(|value| value.get("workspace"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false),
        raw_cargo,
    }
}

pub(crate) fn filetree_root(
    kind: Option<G3RsCargoPolicyRootKind>,
    guardrail_rel_path: Option<String>,
    members_parse_error: bool,
) -> G3RsCargoFileTreeRoot {
    G3RsCargoFileTreeRoot {
        kind,
        rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        guardrail_rel_path,
        members_parse_error,
    }
}

pub(crate) fn input_failure(rel_path: impl Into<String>, message: impl Into<String>) -> G3RsCargoInputFailure {
    G3RsCargoInputFailure {
        rel_path: rel_path.into(),
        message: message.into(),
    }
}

pub(crate) fn missing_member(member_rel: String) -> G3RsCargoMissingMember {
    G3RsCargoMissingMember {
        workspace_root_rel: String::new(),
        workspace_cargo_rel_path: "Cargo.toml".to_owned(),
        member_rel,
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct GuardrailState {
    pub(crate) profile_name: Option<String>,
    pub(crate) escape_hatches: Vec<G3RsCargoEscapeHatch>,
    pub(crate) parse_error: bool,
}

#[derive(Debug, Clone, Default)]
struct StringFieldSnapshot {
    value: Option<String>,
    invalid: bool,
}

fn root_package_field(
    raw_cargo: &toml::Value,
    kind: G3RsCargoPolicyRootKind,
    field: &str,
) -> StringFieldSnapshot {
    if kind == G3RsCargoPolicyRootKind::WorkspaceRoot {
        let workspace_package = string_field(
            raw_cargo
                .get("workspace")
                .and_then(|value| value.get("package")),
            field,
        );
        if workspace_package.value.is_some() || workspace_package.invalid {
            workspace_package
        } else {
            package_field(raw_cargo, field)
        }
    } else {
        package_field(raw_cargo, field)
    }
}

fn package_field(raw_cargo: &toml::Value, field: &str) -> StringFieldSnapshot {
    string_field(raw_cargo.get("package"), field)
}

fn string_field(table: Option<&toml::Value>, field: &str) -> StringFieldSnapshot {
    let Some(value) = table.and_then(|table| table.get(field)) else {
        return StringFieldSnapshot::default();
    };

    match value {
        toml::Value::String(field_value) => StringFieldSnapshot {
            value: Some(field_value.clone()),
            invalid: false,
        },
        _ => StringFieldSnapshot {
            value: None,
            invalid: true,
        },
    }
}

pub(crate) fn profile_name_from_guardrail(raw: &toml::Value) -> Result<Option<String>, ()> {
    let Some(profile) = raw.get("profile") else {
        return Ok(None);
    };
    let Some(name) = profile.get("name") else {
        return Ok(None);
    };

    match name {
        toml::Value::String(value) => Ok(Some(value.clone())),
        _ => Err(()),
    }
}

pub(crate) fn escape_hatches_from_guardrail(
    raw: &toml::Value,
) -> Result<Vec<G3RsCargoEscapeHatch>, ()> {
    let Some(entries) = raw.get("escape_hatches") else {
        return Ok(Vec::new());
    };
    let Some(entries) = entries.as_array() else {
        return Err(());
    };

    entries
        .iter()
        .map(|entry| {
            let table = entry.as_table().ok_or(())?;
            Ok(G3RsCargoEscapeHatch {
                family: table.get("family").and_then(toml::Value::as_str).ok_or(())?.to_owned(),
                file: table.get("file").and_then(toml::Value::as_str).ok_or(())?.to_owned(),
                kind: table.get("kind").and_then(toml::Value::as_str).ok_or(())?.to_owned(),
                selector: table.get("selector").and_then(toml::Value::as_str).ok_or(())?.to_owned(),
                reason: table.get("reason").and_then(toml::Value::as_str).ok_or(())?.to_owned(),
            })
        })
        .collect()
}
