use std::collections::BTreeSet;

use crate::domain::project_tree::ProjectTree;
use crate::domain::report::Severity;

use super::super::super::check as family_check;
use super::super::super::test_support::{dir_entry, project_tree};

fn dev_graph_tree() -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api", "worker"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "apps/api/crates",
                dir_entry(&["app", "domain", "ports", "adapters"], &[]),
            ),
            ("apps/api/crates/app", dir_entry(&["core"], &[])),
            ("apps/api/crates/app/core", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/domain", dir_entry(&["types"], &[])),
            (
                "apps/api/crates/domain/types",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/api/crates/ports", dir_entry(&["repo"], &[])),
            (
                "apps/api/crates/ports/repo",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/api/crates/adapters", dir_entry(&["http"], &[])),
            (
                "apps/api/crates/adapters/http",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/worker", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "apps/worker/crates",
                dir_entry(&["app", "domain", "ports", "adapters"], &[]),
            ),
            ("apps/worker/crates/app", dir_entry(&["core"], &[])),
            (
                "apps/worker/crates/app/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/worker/crates/domain", dir_entry(&["types"], &[])),
            (
                "apps/worker/crates/domain/types",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/worker/crates/ports", dir_entry(&["repo"], &[])),
            (
                "apps/worker/crates/ports/repo",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/worker/crates/adapters", dir_entry(&["http"], &[])),
            (
                "apps/worker/crates/adapters/http",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/app/core\", \"crates/domain/types\", \"crates/ports/repo\", \"crates/adapters/http\"]\n",
            ),
            (
                "apps/api/crates/app/core/Cargo.toml",
                "[package]\nname = \"api-app-core\"\nversion = \"0.1.0\"\n[dev-dependencies]\napi-domain-types = { path = \"../../domain/types\" }\n",
            ),
            (
                "apps/api/crates/domain/types/Cargo.toml",
                "[package]\nname = \"api-domain-types\"\nversion = \"0.1.0\"\n[dev-dependencies]\napi-adapters-http = { path = \"../../adapters/http\" }\n[target.'cfg(unix)'.dev-dependencies]\napi-adapters-http = { path = \"../../adapters/http\" }\n",
            ),
            (
                "apps/api/crates/ports/repo/Cargo.toml",
                "[package]\nname = \"api-ports-repo\"\nversion = \"0.1.0\"\n[dev-dependencies]\napi-adapters-http = { path = \"../../adapters/http\" }\n",
            ),
            (
                "apps/api/crates/adapters/http/Cargo.toml",
                "[package]\nname = \"api-adapters-http\"\nversion = \"0.1.0\"\n[dev-dependencies]\napi-domain-types = { path = \"../../domain/types\" }\n",
            ),
            (
                "apps/worker/Cargo.toml",
                "[workspace]\nmembers = [\"crates/app/core\", \"crates/domain/types\", \"crates/ports/repo\", \"crates/adapters/http\"]\n",
            ),
            (
                "apps/worker/crates/app/core/Cargo.toml",
                "[package]\nname = \"worker-app-core\"\nversion = \"0.1.0\"\n[dev-dependencies]\nworker-domain-types = { path = \"../../domain/types\" }\n",
            ),
            (
                "apps/worker/crates/domain/types/Cargo.toml",
                "[package]\nname = \"worker-domain-types\"\nversion = \"0.1.0\"\n[dev-dependencies]\nworker-adapters-http = { path = \"../../adapters/http\" }\n",
            ),
            (
                "apps/worker/crates/ports/repo/Cargo.toml",
                "[package]\nname = \"worker-ports-repo\"\nversion = \"0.1.0\"\n[dev-dependencies]\nworker-app-core = { path = \"../../app/core\" }\n",
            ),
            (
                "apps/worker/crates/adapters/http/Cargo.toml",
                "[package]\nname = \"worker-adapters-http\"\nversion = \"0.1.0\"\n[dev-dependencies]\nworker-domain-types = { path = \"../../domain/types\" }\n",
            ),
        ],
    )
}

#[test]
fn direct_dev_edges_are_warned_while_target_dev_edges_are_left_to_rule_25() {
    let tree = dev_graph_tree();
    let results = family_check(&tree);

    let rule20 = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-20")
        .collect::<Vec<_>>();
    let actual_rule20_files = rule20
        .iter()
        .filter_map(|result| result.file.clone())
        .collect::<BTreeSet<_>>();
    let actual_rule20_messages = rule20
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_rule20_files = [
        "apps/api/crates/domain/types/Cargo.toml".to_owned(),
        "apps/api/crates/ports/repo/Cargo.toml".to_owned(),
        "apps/worker/crates/domain/types/Cargo.toml".to_owned(),
        "apps/worker/crates/ports/repo/Cargo.toml".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    let expected_rule20_messages = [
        "domain crate `api-domain-types` dev-depends on adapters crate `api-adapters-http` via `dev-dependencies`.".to_owned(),
        "ports crate `api-ports-repo` dev-depends on adapters crate `api-adapters-http` via `dev-dependencies`.".to_owned(),
        "domain crate `worker-domain-types` dev-depends on adapters crate `worker-adapters-http` via `dev-dependencies`.".to_owned(),
        "ports crate `worker-ports-repo` dev-depends on app crate `worker-app-core` via `dev-dependencies`.".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_rule20_files, expected_rule20_files,
        "unexpected RS-HEXARCH-20 hit set: {rule20:#?}"
    );
    assert_eq!(
        actual_rule20_messages, expected_rule20_messages,
        "unexpected RS-HEXARCH-20 messages: {rule20:#?}"
    );
    assert_eq!(
        rule20.len(),
        4,
        "RS-HEXARCH-20 should only own the four direct dev violations: {rule20:#?}"
    );
    assert!(
        rule20
            .iter()
            .all(|result| result.severity == Severity::Warn),
        "RS-HEXARCH-20 should warn, not error: {rule20:#?}"
    );

    let rule25 = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-25")
        .collect::<Vec<_>>();
    let actual_rule25_files = rule25
        .iter()
        .filter_map(|result| result.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_rule25_files = ["apps/api/crates/domain/types/Cargo.toml".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_rule25_files, expected_rule25_files,
        "target dev-dependencies should be owned by RS-HEXARCH-25: {rule25:#?}"
    );
    assert_eq!(
        rule25.len(),
        1,
        "expected one target-dev hit for RS-HEXARCH-25: {rule25:#?}"
    );
    assert!(
        rule25
            .iter()
            .all(|result| result.severity == Severity::Error),
        "RS-HEXARCH-25 should error on the target-dev edge: {rule25:#?}"
    );
}
