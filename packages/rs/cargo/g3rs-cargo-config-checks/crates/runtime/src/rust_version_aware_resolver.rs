use cargo_toml_parser::types::CargoToml;
use g3rs_cargo_types::{G3RsCargoConfigTomlState, G3RsCargoPolicyRoot};
use guardrail3_check_types::G3CheckResult;

use crate::support::{CargoRole, cargo_role, error, info, workspace_resolver};

/// I D const.
const ID: &str = "g3rs-cargo/rust-version-aware-resolver";

/// check fn.
pub(crate) fn check(
    root: &G3RsCargoPolicyRoot,
    cargo: &CargoToml,
    results: &mut Vec<G3CheckResult>,
) {
    if cargo_role(cargo) != CargoRole::WorkspaceRoot {
        return;
    }

    match workspace_resolver(cargo) {
        Some("3") => results.push(info(
            ID,
            "workspace resolver is rust-version aware",
            "`resolver = \"3\"` makes Cargo consider package `rust-version` during dependency resolution.".to_owned(),
            &root.cargo_rel_path,
        )),
        Some("2") => check_resolver_two_config(root, results),
        Some(other) => results.push(error(
            ID,
            "workspace resolver is not rust-version aware",
            format!(
                "`{}` sets resolver `{other}`. Use `resolver = \"3\"`, or use \
                 `resolver = \"2\"` with `.cargo/config.toml` containing \
                 `[resolver] incompatible-rust-versions = \"fallback\"`.",
                root.cargo_rel_path
            ),
            &root.cargo_rel_path,
        )),
        None => results.push(error(
            ID,
            "workspace resolver is not rust-version aware",
            format!(
                "`{}` must set `resolver = \"3\"`, or set `resolver = \"2\"` with \
                 `.cargo/config.toml` containing `[resolver] incompatible-rust-versions = \"fallback\"`.",
                root.cargo_rel_path
            ),
            &root.cargo_rel_path,
        )),
    }
}

/// Validate the fallback config required when a workspace stays on resolver 2.
fn check_resolver_two_config(root: &G3RsCargoPolicyRoot, results: &mut Vec<G3CheckResult>) {
    match &root.cargo_config {
        G3RsCargoConfigTomlState::Parsed {
            rel_path,
            incompatible_rust_versions: Some(value),
        } if value == "fallback" => results.push(info(
            ID,
            "resolver 2 uses rust-version fallback",
            format!(
                "`{}` uses `resolver = \"2\"`, and `{rel_path}` sets \
                 `[resolver] incompatible-rust-versions = \"fallback\"`.",
                root.cargo_rel_path
            ),
            &root.cargo_rel_path,
        )),
        G3RsCargoConfigTomlState::Parsed {
            rel_path,
            incompatible_rust_versions,
        } => results.push(error(
            ID,
            "resolver 2 lacks rust-version fallback",
            format!(
                "`{}` uses `resolver = \"2\"`, so `{rel_path}` must set \
                 `[resolver] incompatible-rust-versions = \"fallback\"`; found `{}`.",
                root.cargo_rel_path,
                incompatible_rust_versions
                    .as_deref()
                    .unwrap_or("missing")
            ),
            rel_path,
        )),
        G3RsCargoConfigTomlState::Missing => results.push(error(
            ID,
            "resolver 2 lacks rust-version fallback",
            format!(
                "`{}` uses `resolver = \"2\"`. Add `.cargo/config.toml` with \
                 `[resolver] incompatible-rust-versions = \"fallback\"`, or move the workspace to \
                 `resolver = \"3\"`.",
                root.cargo_rel_path
            ),
            &root.cargo_rel_path,
        )),
        G3RsCargoConfigTomlState::Unreadable { rel_path, reason }
        | G3RsCargoConfigTomlState::ParseError { rel_path, reason } => results.push(error(
            ID,
            "resolver 2 fallback config is unreadable",
            format!(
                "`{}` uses `resolver = \"2\"`, but `{rel_path}` could not be read as Cargo config: {reason}.",
                root.cargo_rel_path
            ),
            rel_path,
        )),
    }
}
