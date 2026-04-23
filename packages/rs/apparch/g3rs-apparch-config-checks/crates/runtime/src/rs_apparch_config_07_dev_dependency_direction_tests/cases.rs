use g3rs_apparch_config_checks_assertions::rs_apparch_config_07_dev_dependency_direction as assertions;
use g3rs_apparch_types::G3RsApparchDependencyEdge;
use g3rs_apparch_types::G3RsApparchDependencyKind;

use super::helpers::{input, run_rule};

#[test]
fn forbidden_dev_dependency_warns() {
    let results = run_rule(
        &input(Some(G3RsApparchDependencyEdge {
            from_cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
            to_cargo_rel_path: "io/outbound/db/Cargo.toml".to_owned(),
            dep_name: "db-outbound".to_owned(),
            kind: G3RsApparchDependencyKind::TargetDevDependency,
        })),
        "logic/service/Cargo.toml",
    );

    assertions::assert_direction_warning(
        &results,
        "logic/service/Cargo.toml",
        "db",
        "target.*.dev-dependencies",
    );
}

#[test]
fn runtime_dependency_is_not_reported_by_dev_rule() {
    let results = run_rule(
        &input(Some(G3RsApparchDependencyEdge {
            from_cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
            to_cargo_rel_path: "io/outbound/db/Cargo.toml".to_owned(),
            dep_name: "db-outbound".to_owned(),
            kind: G3RsApparchDependencyKind::Dependency,
        })),
        "logic/service/Cargo.toml",
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn allowed_dev_dependency_stays_quiet() {
    let results = run_rule(
        &input(Some(G3RsApparchDependencyEdge {
            from_cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
            to_cargo_rel_path: "types/core/Cargo.toml".to_owned(),
            dep_name: "types-core".to_owned(),
            kind: G3RsApparchDependencyKind::DevDependency,
        })),
        "logic/service/Cargo.toml",
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn package_internal_runtime_dev_dependency_stays_quiet() {
    let results = run_rule(
        &input(Some(G3RsApparchDependencyEdge {
            from_cargo_rel_path: "logic/validate-command/crates/runtime/Cargo.toml".to_owned(),
            to_cargo_rel_path: "logic/validate-command/crates/assertions/Cargo.toml".to_owned(),
            dep_name: "validate-command-assertions".to_owned(),
            kind: G3RsApparchDependencyKind::DevDependency,
        })),
        "logic/validate-command/crates/runtime/Cargo.toml",
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn package_internal_outbound_runtime_dev_dependency_stays_quiet() {
    let results = run_rule(
        &input(Some(G3RsApparchDependencyEdge {
            from_cargo_rel_path: "io/outbound/report/crates/runtime/Cargo.toml".to_owned(),
            to_cargo_rel_path: "io/outbound/report/crates/assertions/Cargo.toml".to_owned(),
            dep_name: "report-assertions".to_owned(),
            kind: G3RsApparchDependencyKind::TargetDevDependency,
        })),
        "io/outbound/report/crates/runtime/Cargo.toml",
    );

    assertions::assert_no_findings(&results);
}
