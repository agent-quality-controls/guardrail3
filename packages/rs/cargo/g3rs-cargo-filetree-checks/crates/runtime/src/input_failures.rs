use g3rs_cargo_types::{G3RsCargoInputFailure, G3RsCargoPolicyRootKind};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-cargo/input-failures";

pub(crate) fn check(input: &G3RsCargoInputFailure, results: &mut Vec<G3CheckResult>) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "failed to read Cargo configuration".to_owned(),
        input.message.clone(),
        Some(input.rel_path.clone()),
        None,
    ));
}

pub(crate) fn check_inventory(
    kind: Option<G3RsCargoPolicyRootKind>,
    cargo_rel_path: &str,
    rust_policy_rel_path: Option<&str>,
    input_failures: &[G3RsCargoInputFailure],
    results: &mut Vec<G3CheckResult>,
) {
    let Some(_kind) = kind else {
        return;
    };

    let has_input_failures = input_failures.iter().any(|failure| {
        failure.rel_path == cargo_rel_path
            || Some(failure.rel_path.as_str()) == rust_policy_rel_path
    }) || input_failures
        .iter()
        .any(|failure| failure.rel_path.ends_with("/Cargo.toml"));

    if has_input_failures {
        return;
    }

    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Info,
            "cargo-family inputs parsed cleanly".to_owned(),
            "Active Cargo policy inputs parsed without cargo-family input failures.".to_owned(),
            Some(cargo_rel_path.to_owned()),
            None,
        )
        .into_inventory(),
    );
}
