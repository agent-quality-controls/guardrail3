use g3rs_topology_file_tree_checks_assertions::required_inputs_fail_closed::rule as assertions;
use g3rs_topology_types::G3RsTopologyFileTreeInputFailure;

use super::super::check;

#[test]
fn descendant_manifest_failure_fires() {
    let mut results = Vec::new();
    let input = G3RsTopologyFileTreeInputFailure {
        rel_path: "bad/Cargo.toml".to_owned(),
        message: "parse failed".to_owned(),
    };

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Rust topology required input failed closed",
            "parse failed",
            Some("bad/Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn multiple_failures_emit_one_result_each() {
    let mut results = Vec::new();

    check(
        &G3RsTopologyFileTreeInputFailure {
            rel_path: "bad/Cargo.toml".to_owned(),
            message: "parse failed".to_owned(),
        },
        &mut results,
    );
    check(
        &G3RsTopologyFileTreeInputFailure {
            rel_path: "worse/Cargo.toml".to_owned(),
            message: "file is not readable".to_owned(),
        },
        &mut results,
    );

    assert_eq!(assertions::findings(&results).len(), 2);
    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Rust topology required input failed closed",
            "parse failed",
            Some("bad/Cargo.toml"),
            false,
        ),
    );
    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Rust topology required input failed closed",
            "file is not readable",
            Some("worse/Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn arbitrary_failure_payload_is_reported() {
    let mut results = Vec::new();
    let input = G3RsTopologyFileTreeInputFailure {
        rel_path: "missing/Cargo.toml".to_owned(),
        message: "parse failed".to_owned(),
    };

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Rust topology required input failed closed",
            "parse failed",
            Some("missing/Cargo.toml"),
            false,
        ),
    );
}
