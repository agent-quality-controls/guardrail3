use std::path::Path;

use guardrail3_app_rs_family_mapper::RsFmtRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_domain_config::types::EscapeHatchConfig;
use guardrail3_domain_project_tree::ProjectTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustfmtConfigKind {
    RustfmtToml,
    DotRustfmtToml,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CargoEditionState {
    Present(String),
    MissingManifest,
    ParseError,
    MissingEdition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolchainChannelState {
    Present(String),
    MissingManifest,
    ParseError,
    MissingChannel,
}

#[derive(Debug, Clone)]
pub struct RustfmtFacts {
    pub(crate) root_config_rel: Option<String>,
    pub(crate) root_parsed: Option<toml::Value>,
    pub(crate) escape_hatches: Vec<EscapeHatchConfig>,
    pub(crate) extra_config_rels: Vec<String>,
    pub(crate) dual_file_conflict_dirs: Vec<String>,
    pub(crate) cargo_edition: CargoEditionState,
    pub(crate) toolchain_channel: ToolchainChannelState,
}

pub fn collect(tree: &ProjectTree, route: &RsFmtRoute) -> RustfmtFacts {
    let mut root_config_rel = None;
    let mut root_parsed = None;
    let extra_config_rels = Vec::new();
    let mut dual_file_conflict_dirs = Vec::new();

    let has_root_rustfmt = route.family_files().iter().any(|file| {
        file.kind() == RustFamilyFileKind::RustfmtToml && file.logical_owner_rel().is_empty()
    });
    let has_root_dot_rustfmt = route.family_files().iter().any(|file| {
        file.kind() == RustFamilyFileKind::DotRustfmtToml && file.logical_owner_rel().is_empty()
    });

    if has_root_rustfmt && has_root_dot_rustfmt {
        dual_file_conflict_dirs.push(String::new());
    }
    if has_root_rustfmt {
        root_config_rel = Some("rustfmt.toml".to_owned());
    } else if has_root_dot_rustfmt {
        root_config_rel = Some(".rustfmt.toml".to_owned());
    }

    if let Some(rel) = &root_config_rel {
        root_parsed = tree
            .file_content(rel)
            .and_then(|content| toml::from_str::<toml::Value>(content).ok())
            .filter(toml::Value::is_table);
    }

    let escape_hatches = tree
        .file_content("guardrail3.toml")
        .and_then(|content| {
            toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok()
        })
        .map(|config| config.escape_hatches().to_vec())
        .unwrap_or_default();

    RustfmtFacts {
        root_config_rel,
        root_parsed,
        escape_hatches,
        extra_config_rels,
        dual_file_conflict_dirs,
        cargo_edition: read_workspace_edition(tree),
        toolchain_channel: read_toolchain_channel(tree),
    }
}

fn read_workspace_edition(tree: &ProjectTree) -> CargoEditionState {
    let Some(content) = tree.file_content("Cargo.toml") else {
        return CargoEditionState::MissingManifest;
    };
    let Ok(parsed) = toml::from_str::<toml::Value>(content) else {
        return CargoEditionState::ParseError;
    };

    parsed
        .get("workspace")
        .and_then(|value| value.get("package"))
        .and_then(|value| value.get("edition"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
        .or_else(|| {
            parsed
                .get("package")
                .and_then(|value| value.get("edition"))
                .and_then(toml::Value::as_str)
                .map(str::to_owned)
        })
        .map_or(
            CargoEditionState::MissingEdition,
            CargoEditionState::Present,
        )
}

fn read_toolchain_channel(tree: &ProjectTree) -> ToolchainChannelState {
    let Some(content) = tree.file_content("rust-toolchain.toml") else {
        return ToolchainChannelState::MissingManifest;
    };
    let Ok(parsed) = toml::from_str::<toml::Value>(content) else {
        return ToolchainChannelState::ParseError;
    };

    parsed
        .get("toolchain")
        .and_then(|value| value.get("channel"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
        .map_or(
            ToolchainChannelState::MissingChannel,
            ToolchainChannelState::Present,
        )
}

pub fn file_name_kind(path: &str) -> RustfmtConfigKind {
    match Path::new(path).file_name().and_then(|name| name.to_str()) {
        Some(".rustfmt.toml") => RustfmtConfigKind::DotRustfmtToml,
        _ => RustfmtConfigKind::RustfmtToml,
    }
}
