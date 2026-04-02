use std::collections::BTreeMap;
use std::path::PathBuf;

use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};
use guardrail3_domain_modules::clippy::build_clippy_toml;

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|dir| (*dir).to_owned()).collect(),
        files.iter().map(|file| (*file).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}

pub fn project_tree(structure: Vec<(&str, DirEntry)>, content: Vec<(&str, String)>) -> ProjectTree {
    let full_structure: BTreeMap<_, _> = structure
        .into_iter()
        .map(|(rel, entry)| (rel.to_owned(), entry))
        .collect();
    let full_content: BTreeMap<_, _> = content
        .into_iter()
        .map(|(rel, content)| (rel.to_owned(), content))
        .collect();
    ProjectTree::build(
        PathBuf::from("/tmp/project"),
        &full_structure,
        &full_content,
        &["".to_owned()],
        &[],
        &[],
        None,
    )
}

pub fn build_fixture_clippy_toml(
    profile_name: &str,
    is_pure_layer: bool,
    garde_enabled: bool,
    extra_methods: &str,
    extra_types: &str,
) -> String {
    build_clippy_toml(
        profile_name,
        is_pure_layer,
        garde_enabled,
        extra_methods,
        extra_types,
    )
}

pub fn root_workspace_tree(clippy_toml: impl Into<String>) -> ProjectTree {
    project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            ("clippy.toml", clippy_toml.into()),
        ],
    )
}

pub fn root_workspace_tree_with_guardrail(
    clippy_toml: impl Into<String>,
    guardrail_toml: impl Into<String>,
) -> ProjectTree {
    project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
        )],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            ("guardrail3.toml", guardrail_toml.into()),
            ("clippy.toml", clippy_toml.into()),
        ],
    )
}

pub fn nested_workspace_member_shadow_tree(file_name: &str) -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["workspace"], &[])),
            (
                "workspace",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("workspace/crates", dir_entry(&["core"], &[])),
            (
                "workspace/crates/core",
                dir_entry(&[], &["Cargo.toml", file_name]),
            ),
        ],
        vec![
            (
                "workspace/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "workspace/crates/core/Cargo.toml",
                "[package]\nname = \"core\"".to_owned(),
            ),
            (
                "workspace/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                &format!("workspace/crates/core/{file_name}"),
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
        ],
    )
}

pub fn same_root_dual_config_tree() -> ProjectTree {
    project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "clippy.toml", ".clippy.toml"]),
        )],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            ("clippy.toml", "max-struct-bools = 3".to_owned()),
            (".clippy.toml", "max-struct-bools = 4".to_owned()),
        ],
    )
}

pub fn incomplete_workspace_policy_root_tree() -> ProjectTree {
    project_tree(
        vec![
            ("", dir_entry(&["apps"], &["clippy.toml"])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            ("apps/backend/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"".to_owned(),
            ),
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/clippy.toml",
                r#"
too-many-lines-threshold = 75
disallowed-methods = []
disallowed-types = []
disallowed-macros = []
"#
                .to_owned(),
            ),
        ],
    )
}

pub fn library_workspace_root_tree(local_clippy_toml: impl Into<String>) -> ProjectTree {
    project_tree(
        vec![
            (
                "",
                dir_entry(&["apps"], &["guardrail3.toml", "clippy.toml"]),
            ),
            ("apps", dir_entry(&["libsite"], &[])),
            (
                "apps/libsite",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/libsite/crates", dir_entry(&["core"], &[])),
            ("apps/libsite/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.apps.libsite]\ntype = \"library\"".to_owned(),
            ),
            (
                "apps/libsite/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/libsite/crates/core/Cargo.toml",
                "[package]\nname = \"core\"".to_owned(),
            ),
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            ("apps/libsite/clippy.toml", local_clippy_toml.into()),
        ],
    )
}

pub fn published_library_workspace_root_tree(local_clippy_toml: impl Into<String>) -> ProjectTree {
    project_tree(
        vec![
            (
                "",
                dir_entry(&["apps"], &["guardrail3.toml", "clippy.toml"]),
            ),
            ("apps", dir_entry(&["libsite"], &[])),
            (
                "apps/libsite",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/libsite/crates", dir_entry(&["core"], &[])),
            ("apps/libsite/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.apps.libsite]\ntype = \"library\"".to_owned(),
            ),
            (
                "apps/libsite/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/libsite/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\npublish = true\n".to_owned(),
            ),
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            ("apps/libsite/clippy.toml", local_clippy_toml.into()),
        ],
    )
}

pub fn published_library_package_root_tree(local_clippy_toml: impl Into<String>) -> ProjectTree {
    project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
        )],
        vec![
            (
                "Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"libcrate\"\npublish = true\n"
                    .to_owned(),
            ),
            (
                "guardrail3.toml",
                "[profile]\nname = \"library\"\n".to_owned(),
            ),
            ("clippy.toml", local_clippy_toml.into()),
        ],
    )
}

pub fn package_library_workspace_root_tree(local_clippy_toml: impl Into<String>) -> ProjectTree {
    project_tree(
        vec![
            (
                "",
                dir_entry(&["packages"], &["guardrail3.toml", "clippy.toml"]),
            ),
            ("packages", dir_entry(&["shared-types"], &[])),
            (
                "packages/shared-types",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("packages/shared-types/crates", dir_entry(&["core"], &[])),
            (
                "packages/shared-types/crates/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.packages]\ntype = \"library\"\n".to_owned(),
            ),
            (
                "packages/shared-types/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\n".to_owned(),
            ),
            (
                "packages/shared-types/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "packages/shared-types/clippy.toml",
                local_clippy_toml.into(),
            ),
        ],
    )
}

pub fn garde_disabled_root_tree(clippy_toml: impl Into<String>) -> ProjectTree {
    root_workspace_tree_with_guardrail(
        clippy_toml,
        "[profile]\nname = \"service\"\n[rust.checks]\ngarde = false",
    )
}

pub fn root_workspace_tree_with_cargo_config(
    cargo_config_rel: &str,
    cargo_config: impl Into<String>,
) -> ProjectTree {
    let cargo_config = cargo_config.into();
    project_tree(
        vec![
            ("", dir_entry(&[".cargo"], &["Cargo.toml", "clippy.toml"])),
            (".cargo", dir_entry(&[], &[cargo_config_rel])),
        ],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (&format!(".cargo/{cargo_config_rel}"), cargo_config),
        ],
    )
}

pub fn nested_workspace_root_with_cargo_config(
    cargo_config_rel: &str,
    cargo_config: impl Into<String>,
) -> ProjectTree {
    let cargo_config = cargo_config.into();
    project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&[".cargo", "crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/backend/.cargo", dir_entry(&[], &[cargo_config_rel])),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            ("apps/backend/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                &format!("apps/backend/.cargo/{cargo_config_rel}"),
                cargo_config,
            ),
        ],
    )
}

pub fn nested_workspace_member_with_cargo_config(
    cargo_config_rel: &str,
    cargo_config: impl Into<String>,
) -> ProjectTree {
    let cargo_config = cargo_config.into();
    project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            (
                "apps/backend/crates/core",
                dir_entry(&[".cargo"], &["Cargo.toml"]),
            ),
            (
                "apps/backend/crates/core/.cargo",
                dir_entry(&[], &[cargo_config_rel]),
            ),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                &format!("apps/backend/crates/core/.cargo/{cargo_config_rel}"),
                cargo_config,
            ),
        ],
    )
}

pub fn unrelated_nested_cargo_config_tree(
    cargo_config_rel: &str,
    cargo_config: impl Into<String>,
) -> ProjectTree {
    let cargo_config = cargo_config.into();
    project_tree(
        vec![
            ("", dir_entry(&["apps", "docs"], &[])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            ("apps/backend/crates/core", dir_entry(&[], &["Cargo.toml"])),
            ("docs", dir_entry(&["guide"], &[])),
            ("docs/guide", dir_entry(&[".cargo"], &[])),
            ("docs/guide/.cargo", dir_entry(&[], &[cargo_config_rel])),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                &format!("docs/guide/.cargo/{cargo_config_rel}"),
                cargo_config,
            ),
        ],
    )
}
