use crate::{CheckResult, Severity};

use crate::inputs::RootTestInput;

const ID: &str = "RS-TEST-11";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_mutants_installed {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "cargo-mutants installed".to_owned(),
                "`cargo-mutants` is available on PATH.".to_owned(),
                Some(input.root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "cargo-mutants missing".to_owned(),
            "`cargo-mutants` was not found on PATH. Install with `cargo install cargo-mutants`."
                .to_owned(),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        ));
    }
}

