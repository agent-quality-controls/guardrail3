use cargo_toml_parser::types::CargoToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{CargoRole, cargo_role, error, info, policy_root_edition, role_label};

const ID: &str = "RS-CARGO-CONFIG-03";

pub(crate) fn check(cargo_rel_path: &str, cargo: &CargoToml, results: &mut Vec<G3CheckResult>) {
    let role = cargo_role(cargo);
    if matches!(role, CargoRole::Other) {
        return;
    }

    match policy_root_edition(cargo) {
        Some(Err(())) => {
            results.push(error(
                ID,
                "edition invalid",
                format!(
                    "`{cargo_rel_path}` must declare edition as a direct string value in `[package]` or `[workspace.package]`."
                ),
                cargo_rel_path,
            ));
        }
        Some(Ok("2024" | "2021")) => {
            results.push(info(
                ID,
                "edition policy satisfied",
                format!(
                    "`{cargo_rel_path}` declares edition `{}`.",
                    policy_root_edition(cargo).expect("edition just matched").expect("string edition")
                ),
                cargo_rel_path,
            ));
        }
        Some(Ok(other)) => match edition_rank(other) {
            Some(_) => results.push(error(
                ID,
                "edition below minimum",
                format!(
                    "`{cargo_rel_path}` declares edition `{other}`. Cargo policy requires edition `2021` or newer."
                ),
                cargo_rel_path,
            )),
            None => results.push(error(
                ID,
                "edition unrecognized",
                format!(
                    "`{cargo_rel_path}` declares unknown edition `{other}`. Use a supported Cargo edition string."
                ),
                cargo_rel_path,
            )),
        },
        None => {
            results.push(error(
                ID,
                "edition missing",
                format!(
                    "`{cargo_rel_path}` must declare an edition for this {}. Add `edition = \"2024\"` to the package metadata.",
                    role_label(role)
                ),
                cargo_rel_path,
            ));
        }
    }
}

fn edition_rank(edition: &str) -> Option<usize> {
    match edition {
        "2015" => Some(0),
        "2018" => Some(1),
        "2021" => Some(2),
        "2024" => Some(3),
        _ => None,
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
