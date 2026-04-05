use std::path::Path;

use cargo_toml_parser::CargoToml;
use guardrail3_app_rs_family_mapper::RsFmtRoute;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_domain_config::types::EscapeHatchConfig;
use rust_toolchain_toml_parser::RustToolchainToml;
use rustfmt_toml_parser::RustfmtToml;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustfmtConfigKind {
    RustfmtToml,
    DotRustfmtToml,
}

#[derive(Debug, Clone)]
pub struct RustfmtFacts {
    pub(crate) root_config_rel: Option<String>,
    pub(crate) root_parsed: Option<RustfmtToml>,
    pub(crate) root_parse_error: Option<String>,
    pub(crate) escape_hatches: Vec<EscapeHatchConfig>,
    pub(crate) extra_config_rels: Vec<String>,
    pub(crate) dual_file_conflict_dirs: Vec<String>,
    pub(crate) cargo_rel_path: String,
    pub(crate) cargo_parsed: Option<CargoToml>,
    pub(crate) cargo_parse_error: Option<String>,
    pub(crate) toolchain_rel_path: String,
    pub(crate) toolchain_parsed: Option<RustToolchainToml>,
    pub(crate) toolchain_parse_error: Option<String>,
}

pub fn collect(tree: &ProjectTree, route: &RsFmtRoute) -> RustfmtFacts {
    let mut root_config_rel = None;
    let mut root_parsed = None;
    let mut root_parse_error = None;
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
        match tree.file_content(rel) {
            Some(content) => match rustfmt_toml_parser::parse(content) {
                Ok(parsed) => root_parsed = Some(parsed),
                Err(parse_error) => root_parse_error = Some(parse_error.to_string()),
            },
            None => {
                root_parse_error = Some("rustfmt config content missing from ProjectTree".to_owned())
            }
        }
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
        root_parse_error,
        escape_hatches,
        extra_config_rels,
        dual_file_conflict_dirs,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_parsed: read_cargo(tree),
        cargo_parse_error: read_cargo_parse_error(tree),
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_parsed: read_toolchain(tree),
        toolchain_parse_error: read_toolchain_parse_error(tree),
    }
}

fn read_cargo(tree: &ProjectTree) -> Option<CargoToml> {
    tree.file_content("Cargo.toml")
        .and_then(|content| cargo_toml_parser::parse(content).ok())
}

fn read_cargo_parse_error(tree: &ProjectTree) -> Option<String> {
    tree.file_content("Cargo.toml").and_then(|content| {
        cargo_toml_parser::parse(content)
            .err()
            .map(|parse_error| parse_error.to_string())
    })
}

fn read_toolchain(tree: &ProjectTree) -> Option<RustToolchainToml> {
    tree.file_content("rust-toolchain.toml")
        .and_then(|content| rust_toolchain_toml_parser::parse(content).ok())
}

fn read_toolchain_parse_error(tree: &ProjectTree) -> Option<String> {
    tree.file_content("rust-toolchain.toml").and_then(|content| {
        rust_toolchain_toml_parser::parse(content)
            .err()
            .map(|parse_error| parse_error.to_string())
    })
}

pub fn file_name_kind(path: &str) -> RustfmtConfigKind {
    match Path::new(path).file_name().and_then(|name| name.to_str()) {
        Some(".rustfmt.toml") => RustfmtConfigKind::DotRustfmtToml,
        _ => RustfmtConfigKind::RustfmtToml,
    }
}
