use std::collections::BTreeSet;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_14_dependency_inventory as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn fixture_backed_path_dependencies_are_inventoried_with_exact_messages() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/app/queries/Cargo.toml",
        "[package]\nname = \"backend-app-queries\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../../domain/types\" }\n\n[dev-dependencies]\nshared-types = { path = \"../../../../../packages/shared-types\" }\n\n[build-dependencies]\nbackend-ports-outbound-repo = { path = \"../../ports/outbound/repo\" }\n\n[target.'cfg(unix)'.dependencies]\nbackend-domain-engine-target = { package = \"backend-domain-engine\", path = \"../../domain/engine\" }\n\n[target.'cfg(unix)'.dev-dependencies]\nshared-types-target = { package = \"shared-types\", path = \"../../../../../packages/shared-types\" }\n\n[target.'cfg(unix)'.build-dependencies]\nbackend-ports-outbound-events-target = { package = \"backend-ports-outbound-events\", path = \"../../ports/outbound/events\" }\n",
    );

    let results = assertions::run_family(tmp.path());
    let actual_messages = results
        .iter()
        .filter(|result| {
            result.id == "RS-HEXARCH-14"
                && result.file.as_deref() == Some("apps/backend/crates/app/queries/Cargo.toml")
        })
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = [
        "`apps/backend/crates/app/queries` depends on `backend-domain-types` via `dependencies` resolved to `apps/backend/crates/domain/types`.".to_owned(),
        "`apps/backend/crates/app/queries` depends on `shared-types` via `dev-dependencies` resolved to `packages/shared-types`.".to_owned(),
        "`apps/backend/crates/app/queries` depends on `backend-ports-outbound-repo` via `build-dependencies` resolved to `apps/backend/crates/ports/outbound/repo`.".to_owned(),
        "`apps/backend/crates/app/queries` depends on `backend-domain-engine` via `target.*.dependencies` resolved to `apps/backend/crates/domain/engine`.".to_owned(),
        "`apps/backend/crates/app/queries` depends on `shared-types` via `target.*.dev-dependencies` resolved to `packages/shared-types`.".to_owned(),
        "`apps/backend/crates/app/queries` depends on `backend-ports-outbound-events` via `target.*.build-dependencies` resolved to `apps/backend/crates/ports/outbound/events`.".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_messages, expected_messages,
        "unexpected inventory results: {results:#?}"
    );
    assert!(
        results
            .iter()
            .filter(|result| {
                result.id == "RS-HEXARCH-14"
                    && result.file.as_deref() == Some("apps/backend/crates/app/queries/Cargo.toml")
            })
            .all(|result| result.inventory),
        "inventory results should be marked as inventory: {results:#?}"
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

    let results = assertions::run_family(tmp.path());
    let inventory_messages = results
        .iter()
        .filter(|result| {
            result.id == "RS-HEXARCH-14"
                && result.file.as_deref() == Some("apps/backend/crates/app/queries/Cargo.toml")
        })
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = ["`apps/backend/crates/app/queries` depends on `backend-domain-types` via `dependencies` resolved to `apps/backend/crates/domain/types`.".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        inventory_messages, expected_messages,
        "broken paths should not fabricate inventory entries: {results:#?}"
    );
}
