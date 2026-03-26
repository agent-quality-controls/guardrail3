use std::collections::BTreeSet;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_16_patch_replace_bypass as assertions;
use super::{copy_fixture, write_file};

#[test]
fn fixture_backed_patch_and_replace_only_error_for_layered_targets() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    \"crates/domain/engine\",\n    \"crates/app/commands\",\n    \"crates/app/queries\",\n    \"crates/ports/inbound/api\",\n    \"crates/ports/outbound/repo\",\n    \"crates/ports/outbound/events\",\n    \"crates/adapters/inbound/rest\",\n    \"crates/adapters/outbound/postgres\",\n    \"crates/adapters/outbound/queue\",\n]\nresolver = \"2\"\n\n[patch.crates-io]\nbackend-domain-types = { path = \"crates/domain/types\" }\nmissing-layered = { path = \"crates/domain/missing\" }\n\n[replace]\n\"backend-domain-engine:0.1.0\" = { path = \"crates/domain/engine\" }\n\"shared-types:0.1.0\" = { path = \"../../packages/shared-types\" }\n",
    );

    let results = super::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "");
    let actual_titles = errors
        .iter()
        .map(|result| result.title.clone())
        .collect::<BTreeSet<_>>();
    let expected_titles = [
        "patch/replace entry `backend-domain-types` bypasses hexarch dependency checks".to_owned(),
        "patch/replace entry `backend-domain-engine:0.1.0` bypasses hexarch dependency checks"
            .to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_titles, expected_titles,
        "unexpected patch/replace hit set: {errors:#?}"
    );
}
