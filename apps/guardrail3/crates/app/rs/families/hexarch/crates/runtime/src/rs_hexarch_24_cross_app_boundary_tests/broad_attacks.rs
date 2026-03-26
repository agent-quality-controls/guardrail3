use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_24_cross_app_boundary as assertions;
use super::{dir_entry, project_tree, run_tree};

#[test]
fn cross_app_edges_error_and_same_app_edges_do_not() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api", "worker"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["app", "domain"], &[])),
            ("apps/api/crates/app", dir_entry(&["core"], &[])),
            ("apps/api/crates/app/core", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/domain", dir_entry(&["types"], &[])),
            (
                "apps/api/crates/domain/types",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/worker", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "apps/worker/crates",
                dir_entry(&["domain", "app", "ports"], &[]),
            ),
            ("apps/worker/crates/domain", dir_entry(&["jobs"], &[])),
            (
                "apps/worker/crates/domain/jobs",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/worker/crates/app", dir_entry(&["processor"], &[])),
            (
                "apps/worker/crates/app/processor",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/worker/crates/ports", dir_entry(&["outbound"], &[])),
            (
                "apps/worker/crates/ports/outbound",
                dir_entry(&["queue"], &[]),
            ),
            (
                "apps/worker/crates/ports/outbound/queue",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\", \"crates/*/*\"]\n",
            ),
            (
                "apps/worker/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\", \"crates/*/*\", \"crates/*/*/*\"]\n",
            ),
            (
                "apps/api/crates/app/core/Cargo.toml",
                "[package]\nname = \"api-app-core\"\n\n[dependencies]\napi-domain-types = { path = \"../../domain/types\" }\nworker-domain-jobs = { path = \"../../../../worker/crates/domain/jobs\" }\n\n[dev-dependencies]\nworker-app-processor = { path = \"../../../../worker/crates/app/processor\" }\n\n[build-dependencies]\nworker-ports-outbound-queue = { path = \"../../../../worker/crates/ports/outbound/queue\" }\n\n[target.'cfg(unix)'.dependencies]\nworker-domain-jobs-target = { path = \"../../../../worker/crates/domain/jobs\" }\n",
            ),
            (
                "apps/api/crates/domain/types/Cargo.toml",
                "[package]\nname = \"api-domain-types\"\n",
            ),
            (
                "apps/worker/crates/domain/jobs/Cargo.toml",
                "[package]\nname = \"worker-domain-jobs\"\n",
            ),
            (
                "apps/worker/crates/app/processor/Cargo.toml",
                "[package]\nname = \"worker-app-processor\"\n",
            ),
            (
                "apps/worker/crates/ports/outbound/queue/Cargo.toml",
                "[package]\nname = \"worker-ports-outbound-queue\"\n",
            ),
        ],
    );

    let results = super::run_tree(&tree);
    assertions::assert_error_file_set(
        &results,
        "",
        4,
        &["apps/api/crates/app/core/Cargo.toml"],
    );
}
