use super::{collected_facts, dir_entry, failure_facts, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_11_input_failures as assertions;

#[test]
fn collect_surfaces_guardrail_parse_failure() {
    let facts = failure_facts(
        "guardrail3.toml",
        "Failed to parse guardrail3.toml for dependency policy resolution.",
    );
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result.message().contains("Failed to parse guardrail3.toml"),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![(Some("guardrail3.toml"), assertions::Severity::Error, true)],
    );
}

#[test]
fn unreadable_guardrail_policy_surfaces_explicit_failure() {
    let facts = failure_facts(
        "guardrail3.toml",
        "Failed to read guardrail3.toml for dependency policy resolution.",
    );
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result
                    .message()
                    .contains("Failed to read guardrail3.toml for dependency policy resolution."),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![(Some("guardrail3.toml"), assertions::Severity::Error, true)],
    );
}

#[test]
fn guardrail_policy_unknown_crate_key_is_ignored_when_owned_fields_are_valid() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"
                    allowd_deps = ["serde"]
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [workspace]
                    members = []

                    [package]
                    name = "api"

                    [dependencies]
                    reqwest = "0.12"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let failure_summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| (result.file(), result.severity()))
        .collect::<Vec<_>>();

    assertions::assert_summary(
        failure_summary,
        Vec::<(Option<&str>, assertions::Severity)>::new(),
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-DEPS-05"
                && result.file() == Some("apps/api/Cargo.toml")
                && result
                    .message()
                    .contains("Dependency `reqwest` in `[dependencies]` is not allowlisted")
        }),
        "expected allowlist rule to stay active: {results:#?}"
    );
}

#[test]
fn guardrail_policy_empty_allowed_dep_entry_surfaces_explicit_failure() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["packages"], &["guardrail3.toml"])),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.packages]
                    profile = "library"
                    allowed_deps = [""]
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [workspace]
                    members = []

                    [package]
                    name = "core"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result
                    .message()
                    .contains("must not contain empty dependency names"),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![(Some("guardrail3.toml"), assertions::Severity::Error, true)],
    );
}

#[test]
fn guardrail_policy_unknown_rust_key_is_ignored_when_owned_fields_are_valid() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["packages"], &["guardrail3.toml"])),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.packages]
                    profile = "library"
                    allowed_deps = ["serde"]

                    [rust.packagess]
                    profile = "library"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [workspace]
                    members = []

                    [package]
                    name = "core"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let failure_summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| (result.file(), result.severity()))
        .collect::<Vec<_>>();

    assertions::assert_summary(
        failure_summary,
        Vec::<(Option<&str>, assertions::Severity)>::new(),
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-DEPS-08"
                && result.file() == Some("packages/core/Cargo.toml")
                && result.inventory()
        }),
        "expected library allowlist coverage to stay active: {results:#?}"
    );
}

#[test]
fn workspace_members_with_non_string_entries_surface_explicit_failure() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["packages"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.packages]
                    profile = "library"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["packages/*", 7]
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [package]
                    name = "core"

                    [dependencies]
                    reqwest = "0.12"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result
                    .message()
                    .contains("`[workspace].members` must contain only strings"),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![
            (Some("Cargo.toml"), assertions::Severity::Error, true),
            (Some("Cargo.toml"), assertions::Severity::Error, false),
            (Some("packages/core/Cargo.toml"), assertions::Severity::Error, false),
        ],
    );
}

#[test]
fn workspace_dependency_package_with_non_string_name_surfaces_explicit_failure() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["packages"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.packages]
                    profile = "library"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["packages/*"]

                    [workspace.dependencies]
                    reqwest = { package = 7 }
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [workspace]
                    members = []

                    [package]
                    name = "core"

                    [dependencies]
                    reqwest = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result
                    .message()
                    .contains("`[workspace.dependencies].reqwest.package` must be a string"),
                result.message().contains("workspace = true"),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![
            (Some("Cargo.toml"), assertions::Severity::Error, true, false),
            (Some("Cargo.toml"), assertions::Severity::Error, false, false),
        ],
    );
}

#[test]
fn dependency_workspace_flag_with_non_boolean_value_surfaces_explicit_failure() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [workspace]
                    members = []

                    [package]
                    name = "api"

                    [dependencies]
                    reqwest = { workspace = "yes" }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result
                    .message()
                    .contains("`[dependencies].reqwest.workspace` must be a boolean"),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![(Some("apps/api/Cargo.toml"), assertions::Severity::Error, true)],
    );
}

#[test]
fn workspace_true_without_workspace_dependency_entry_surfaces_explicit_failure() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["packages"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.packages]
                    profile = "library"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["packages/*"]
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [package]
                    name = "core"

                    [dependencies]
                    serde = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result.message().contains("[workspace.dependencies].serde"),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![(
            Some("packages/core/Cargo.toml"),
            assertions::Severity::Error,
            true,
        )],
    );
}

#[test]
fn unreadable_member_manifest_surfaces_explicit_failure() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["apps/api"]
                "#,
            ),
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result
                    .message()
                    .contains("Failed to read Cargo.toml for dependency root discovery."),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![(Some("apps/api/Cargo.toml"), assertions::Severity::Error, false)],
    );
}

#[test]
fn unreadable_workspace_manifest_surfaces_explicit_failure() {
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        vec![(
            "guardrail3.toml",
            r#"
                [profile]
                name = "service"
            "#,
        )],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result
                    .message()
                    .contains("Failed to read Cargo.toml for dependency root discovery."),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(summary, Vec::<(Option<&str>, assertions::Severity, bool)>::new());
}

#[test]
fn malformed_member_manifest_surfaces_explicit_failure() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["apps/api"]
                "#,
            ),
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"
                "#,
            ),
            ("apps/api/Cargo.toml", "[[broken"),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result
                    .message()
                    .contains("Failed to parse workspace Cargo.toml"),
                result
                    .message()
                    .contains("Failed to parse Cargo.toml for dependency root discovery"),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![(
            Some("apps/api/Cargo.toml"),
            assertions::Severity::Error,
            false,
            false,
        )],
    );
}

#[test]
fn malformed_workspace_manifest_does_not_fail_open_workspace_true_resolution() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("Cargo.toml", "[[broken"),
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"

                    [dependencies]
                    reqwest = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result
                    .message()
                    .contains("Failed to parse workspace Cargo.toml"),
                result
                    .message()
                    .contains("Failed to parse Cargo.toml for dependency root discovery"),
                result.message().contains("workspace = true"),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        Vec::<(Option<&str>, assertions::Severity, bool, bool, bool)>::new(),
    );
}

#[test]
fn unreadable_gitignore_surfaces_explicit_failure() {
    let tree = project_tree(
        vec![(
            "",
            dir_entry(&[], &[".gitignore", "Cargo.toml", "guardrail3.toml"]),
        )],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [profile]
                    name = "service"
                "#,
            ),
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = []
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result
                    .message()
                    .contains("Failed to read `.gitignore` for Cargo.lock masking checks."),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![(Some(".gitignore"), assertions::Severity::Error, true)],
    );
}
