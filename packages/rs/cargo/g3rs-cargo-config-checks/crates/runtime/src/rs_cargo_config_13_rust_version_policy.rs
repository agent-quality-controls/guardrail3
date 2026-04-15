use g3rs_cargo_types::G3RsCargoPolicyRoot;
use guardrail3_rs_toml_parser::RustProfile;
use guardrail3_check_types::G3CheckResult;

use crate::support::{rust_policy_valid, rust_profile};

const ID: &str = "RS-CARGO-CONFIG-13";

pub(crate) fn check(root: &G3RsCargoPolicyRoot, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(root) {
        return;
    }
    if root.rust_version_invalid {
        results.push(crate::support::error(
            ID,
            "rust-version invalid",
            format!(
                "`{}` must declare `rust-version` as a string value when it is present.",
                root.cargo_rel_path
            ),
            &root.cargo_rel_path,
        ));
        return;
    }

    let is_library = rust_profile(root) == Some(RustProfile::Library);
    match (is_library, root.rust_version.as_deref()) {
        (true, Some(version)) => results.push(crate::support::info(
            ID,
            "library rust-version declared",
            format!("`{}` declares `rust-version = \"{version}\"`.", root.cargo_rel_path),
            &root.cargo_rel_path,
        )),
        (true, None) => results.push(crate::support::error(
            ID,
            "library rust-version missing",
            "Library crates must declare `rust-version` (minimum supported Rust version). Add `rust-version = \"1.75\"` (or appropriate version) to `[package]`.",
            &root.cargo_rel_path,
        )),
        (false, Some(version)) => results.push(crate::support::info(
            ID,
            "rust-version inventory",
            format!("`{}` declares `rust-version = \"{version}\"`.", root.cargo_rel_path),
            &root.cargo_rel_path,
        )),
        (false, None) => results.push(crate::support::info(
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
