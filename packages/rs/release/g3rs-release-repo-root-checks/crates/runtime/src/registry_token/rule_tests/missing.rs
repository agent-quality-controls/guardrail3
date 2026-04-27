use g3rs_release_repo_root_checks_assertions::registry_token::rule as assertions;

use super::helpers::run;

#[test]
fn warns_when_registry_token_missing() {
    let results = run(false);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "CARGO_REGISTRY_TOKEN missing from workflows",
            "No workflow structurally wires `CARGO_REGISTRY_TOKEN` into release steps. Add `CARGO_REGISTRY_TOKEN` as a secret in the release workflow.",
            "Cargo.toml",
            false,
        )],
    );
}
