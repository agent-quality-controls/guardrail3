use super::{dir_entry, project_tree, run_tree};
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_25_target_dependency_direction as assertions;

#[test]
fn forbidden_target_sections_error_and_allowed_target_sections_do_not() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "apps/api/crates",
                dir_entry(&["domain", "app", "adapters"], &[]),
            ),
            (
                "apps/api/crates/domain",
                dir_entry(&["types", "helper"], &[]),
            ),
            (
                "apps/api/crates/domain/types",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            (
                "apps/api/crates/domain/helper",
                dir_entry(&[], &["Cargo.toml"]),
            ),
            ("apps/api/crates/app", dir_entry(&["core"], &[])),
            ("apps/api/crates/app/core", dir_entry(&[], &["Cargo.toml"])),
            ("apps/api/crates/adapters", dir_entry(&["http"], &[])),
            (
                "apps/api/crates/adapters/http",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/domain/*\", \"crates/app/*\", \"crates/adapters/*\"]\n",
            ),
            (
                "apps/api/crates/domain/types/Cargo.toml",
                "[package]\nname = \"api-domain-types\"\n[target.'cfg(unix)'.dependencies]\napi-domain-helper = { path = \"../helper\" }\n",
            ),
            (
                "apps/api/crates/domain/helper/Cargo.toml",
                "[package]\nname = \"api-domain-helper\"\n",
            ),
            (
                "apps/api/crates/app/core/Cargo.toml",
                "[package]\nname = \"api-app-core\"\n[target.'cfg(unix)'.dependencies]\napi-adapters-http = { path = \"../../adapters/http\" }\n[target.'cfg(windows)'.dev-dependencies]\napi-adapters-http = { path = \"../../adapters/http\" }\n[target.'cfg(target_os = \"linux\")'.build-dependencies]\napi-adapters-http = { path = \"../../adapters/http\" }\n",
            ),
            (
                "apps/api/crates/adapters/http/Cargo.toml",
                "[package]\nname = \"api-adapters-http\"\n",
            ),
        ],
    );

    let results = super::run_tree(&tree);
    assertions::assert_error_file_set(&results, "", 3, &["apps/api/crates/app/core/Cargo.toml"]);
}
