use super::super::audit_edge_for_test as audit_edge;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_17_workspace_inherited_direction as rule17_assertions;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_18_renamed_dependency_direction as rule18_assertions;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_24_cross_app_boundary as rule24_assertions;
use super::{dir_entry, project_tree};

#[test]
fn version_only_inherited_dep_with_same_name_local_member_stays_out_of_scope() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["domain", "adapters"], &[])),
            ("apps/api/crates/domain", dir_entry(&["core"], &[])),
            (
                "apps/api/crates/domain/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/api/crates/adapters", dir_entry(&["http"], &[])),
            (
                "apps/api/crates/adapters/http",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/domain/*\", \"crates/adapters/*\"]\n[workspace.dependencies]\napi-adapters-http = \"1\"\n",
            ),
            (
                "apps/api/crates/domain/core/Cargo.toml",
                "[package]\nname = \"api-domain-core\"\n[dependencies]\napi-adapters-http = { workspace = true }\n",
            ),
            (
                "apps/api/crates/adapters/http/Cargo.toml",
                "[package]\nname = \"api-adapters-http\"\n",
            ),
        ],
    );

    let audit = audit_edge(&tree, "apps/api/crates/domain/core");

    rule17_assertions::assert_no_error(&audit.rule17, "");
}

#[test]
fn renamed_inherited_path_dep_is_owned_by_rule_17_only() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["domain", "adapters"], &[])),
            ("apps/api/crates/domain", dir_entry(&["core"], &[])),
            (
                "apps/api/crates/domain/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/api/crates/adapters", dir_entry(&["http"], &[])),
            (
                "apps/api/crates/adapters/http",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/domain/*\", \"crates/adapters/*\"]\n[workspace.dependencies]\nrenamed_http = { package = \"api-adapters-http\", path = \"crates/adapters/http\" }\n",
            ),
            (
                "apps/api/crates/domain/core/Cargo.toml",
                "[package]\nname = \"api-domain-core\"\n[dependencies]\nrenamed_http = { workspace = true }\n",
            ),
            (
                "apps/api/crates/adapters/http/Cargo.toml",
                "[package]\nname = \"api-adapters-http\"\n",
            ),
        ],
    );

    let audit = audit_edge(&tree, "apps/api/crates/domain/core");

    rule17_assertions::assert_error_count(&audit.rule17, "", 1);
    rule18_assertions::assert_no_error(&audit.rule18, "");
}

#[test]
fn cross_app_inherited_path_dep_is_owned_by_rule_24_not_rule_17() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["backend", "worker"], &[])),
            ("apps/backend", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", dir_entry(&["domain"], &[])),
            ("apps/backend/crates/domain", dir_entry(&["engine"], &[])),
            (
                "apps/backend/crates/domain/engine",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/worker", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/worker/crates", dir_entry(&["app"], &[])),
            ("apps/worker/crates/app", dir_entry(&["processor"], &[])),
            (
                "apps/worker/crates/app/processor",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/domain/*\"]\n[workspace.dependencies]\nworker-app-processor = { path = \"../worker/crates/app/processor\" }\n",
            ),
            (
                "apps/backend/crates/domain/engine/Cargo.toml",
                "[package]\nname = \"backend-domain-engine\"\n[dependencies]\nworker-app-processor = { workspace = true }\n",
            ),
            (
                "apps/worker/Cargo.toml",
                "[workspace]\nmembers = [\"crates/app/*\"]\n",
            ),
            (
                "apps/worker/crates/app/processor/Cargo.toml",
                "[package]\nname = \"worker-app-processor\"\n",
            ),
        ],
    );

    let audit = audit_edge(&tree, "apps/backend/crates/domain/engine");

    rule17_assertions::assert_no_error(&audit.rule17, "");
    rule24_assertions::assert_error_count(&audit.rule24, "", 1);
}

#[test]
fn allowed_renamed_inherited_path_dep_stays_clean() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["app", "domain"], &[])),
            ("apps/api/crates/app", dir_entry(&["core"], &[])),
            ("apps/api/crates/domain", dir_entry(&["types"], &[])),
            ("apps/api/crates/app/core", dir_entry(&[], &["Cargo.toml"])),
            (
                "apps/api/crates/domain/types",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/app/*\", \"crates/domain/*\"]\n[workspace.dependencies]\nrenamed_types = { package = \"api-domain-types\", path = \"crates/domain/types\" }\n",
            ),
            (
                "apps/api/crates/app/core/Cargo.toml",
                "[package]\nname = \"api-app-core\"\n[dependencies]\nrenamed_types = { workspace = true }\n",
            ),
            (
                "apps/api/crates/domain/types/Cargo.toml",
                "[package]\nname = \"api-domain-types\"\n",
            ),
        ],
    );

    let audit = audit_edge(&tree, "apps/api/crates/app/core");

    rule17_assertions::assert_no_error(&audit.rule17, "");
}
