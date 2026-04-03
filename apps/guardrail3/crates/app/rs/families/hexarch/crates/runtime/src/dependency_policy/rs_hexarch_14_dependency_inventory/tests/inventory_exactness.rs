use std::collections::BTreeSet;

use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::dependency_policy::rs_hexarch_14_dependency_inventory as assertions;

#[test]
fn fixture_backed_path_dependencies_are_inventoried_with_exact_messages() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/app/queries/Cargo.toml",
        "[package]\nname = \"backend-app-queries\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../../domain/types\" }\n\n[dev-dependencies]\nshared-types = { path = \"../../../../../packages/shared-types\" }\n\n[build-dependencies]\nbackend-ports-outbound-repo = { path = \"../../ports/outbound/repo\" }\n\n[target.'cfg(unix)'.dependencies]\nbackend-domain-engine-target = { package = \"backend-domain-engine\", path = \"../../domain/engine\" }\n\n[target.'cfg(unix)'.dev-dependencies]\nshared-types-target = { package = \"shared-types\", path = \"../../../../../packages/shared-types\" }\n\n[target.'cfg(unix)'.build-dependencies]\nbackend-ports-outbound-events-target = { package = \"backend-ports-outbound-events\", path = \"../../ports/outbound/events\" }\n",
    );

    let results = super::run_family(tmp.path());
    let expected_messages = [
        "`apps/backend/crates/app/queries` depends on `backend-domain-types` via `dependencies` resolved to `apps/backend/crates/domain/types`.".to_owned(),
        "`apps/backend/crates/app/queries` depends on `backend-ports-outbound-repo` via `build-dependencies` resolved to `apps/backend/crates/ports/outbound/repo`.".to_owned(),
        "`apps/backend/crates/app/queries` depends on `backend-domain-engine` via `target.*.dependencies` resolved to `apps/backend/crates/domain/engine`.".to_owned(),
        "`apps/backend/crates/app/queries` depends on `backend-ports-outbound-events` via `target.*.build-dependencies` resolved to `apps/backend/crates/ports/outbound/events`.".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assertions::assert_inventory_results(
        &results,
        "apps/backend/crates/app/queries/Cargo.toml",
        4,
        &expected_messages
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    );
}

#[test]
fn broken_path_dependencies_are_not_inventoried() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/app/queries/Cargo.toml",
        "[package]\nname = \"backend-app-queries\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../../domain/types\" }\nmissing-same-app = { path = \"../../app/missing\" }\nmissing-package = { path = \"../../../../../packages/missing\" }\n",
    );

    let results = super::run_family(tmp.path());
    let expected_messages = ["`apps/backend/crates/app/queries` depends on `backend-domain-types` via `dependencies` resolved to `apps/backend/crates/domain/types`.".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assertions::assert_inventory_results(
        &results,
        "apps/backend/crates/app/queries/Cargo.toml",
        1,
        &expected_messages
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    );
}
