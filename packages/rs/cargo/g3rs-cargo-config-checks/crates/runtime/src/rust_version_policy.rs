use cargo_toml_parser::types::CargoStringFieldState;
use g3rs_cargo_types::G3RsCargoPolicyRoot;
use g3rs_toml_parser::types::RustProfile;
use guardrail3_check_types::G3CheckResult;

use crate::support::{rust_policy_valid, rust_profile};

/// I D const.
const ID: &str = "g3rs-cargo/rust-version-policy";

/// check fn.
pub(crate) fn check(root: &G3RsCargoPolicyRoot, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(root) {
        return;
    }
    let is_library = rust_profile(root) == Some(RustProfile::Library);
    match (is_library, crate::support::root_rust_version_state(root)) {
        (_, CargoStringFieldState::WrongType(_)) => results.push(crate::support::error(
            ID,
            "rust-version invalid",
            format!(
                "`{}` must declare `rust-version` as a string value when it is present.",
                root.cargo_rel_path
            ),
            &root.cargo_rel_path,
        )),
        (true, CargoStringFieldState::Value(version)) => results.push(crate::support::info(
            ID,
            "library rust-version declared",
            format!("`{}` declares `rust-version = \"{version}\"`.", root.cargo_rel_path),
            &root.cargo_rel_path,
        )),
        (true, CargoStringFieldState::Missing | CargoStringFieldState::Inherit) => results.push(crate::support::error(
            ID,
            "library rust-version missing",
            "Library crates must declare `rust-version` (minimum supported Rust version). Add `rust-version = \"1.75\"` (or appropriate version) to `[package]`.",
            &root.cargo_rel_path,
        )),
        (false, CargoStringFieldState::Value(version)) => results.push(crate::support::info(
            ID,
            "rust-version inventory",
            format!("`{}` declares `rust-version = \"{version}\"`.", root.cargo_rel_path),
            &root.cargo_rel_path,
        )),
        (false, CargoStringFieldState::Missing | CargoStringFieldState::Inherit) => results.push(crate::support::info(
            ID,
            "rust-version inventory",
            format!(
                "`{}` does not declare `rust-version`. This is optional for non-library crates.",
                root.cargo_rel_path
            ),
            &root.cargo_rel_path,
        )),
    }
}
