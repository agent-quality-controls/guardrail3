use std::collections::BTreeSet;

use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_file_tree_checks_assertions::rs_test_02_owned_sidecar_shape::rule as assertions;

#[test]
fn reports_inventory_for_owned_sidecar_shape() {
    let results = assertions::check(&input(
        vec![
            file(
                "src/lib.rs",
                G3RsTestFileKind::Source,
                Some(""),
                Some("lib"),
                None,
                "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod lib_tests;\n",
            ),
            file(
                "src/lib_tests/mod.rs",
                G3RsTestFileKind::InternalSidecarMod,
                Some(""),
                Some("lib"),
                None,
                "#[test]\nfn keeps_shape() { assert!(true); }\n",
            ),
        ],
        vec![component("", "", Some("demo_runtime"), false, None)],
    ));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-FILETREE-02",
        "owned sidecar shape confirmed",
        "Cargo.toml",
    );
}

#[test]
fn reports_ad_hoc_src_tests_tree() {
    let results = assertions::check(&input(
        vec![file(
            "src/tests/helper.rs",
            G3RsTestFileKind::Source,
            None,
            Some("helper"),
            None,
            "#[test]\nfn stray() { assert!(true); }\n",
        )],
        vec![component("", "", Some("demo_runtime"), false, None)],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-02",
        G3Severity::Error,
        "ad hoc src/tests tree",
        "src/tests",
        None,
    );
}

#[test]
fn reports_nested_ad_hoc_src_tests_tree() {
    let results = assertions::check(&input(
        vec![file(
            "src/foo/tests/helper.rs",
            G3RsTestFileKind::Source,
            None,
            Some("helper"),
            None,
            "#[test]\nfn stray() { assert!(true); }\n",
        )],
        vec![component("", "", Some("demo_runtime"), false, None)],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-02",
        G3Severity::Error,
        "ad hoc src/tests tree",
        "src/foo/tests",
        None,
    );
}

#[test]
fn reports_flat_sidecar_file() {
    let results = assertions::check(&input(
        vec![file(
            "src/lib_tests.rs",
            G3RsTestFileKind::Source,
            Some(""),
            Some("lib_tests"),
            None,
            "#[test]\nfn stray() { assert!(true); }\n",
        )],
        vec![component("", "", Some("demo_runtime"), false, None)],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-02",
        G3Severity::Error,
        "flat sidecar test file",
        "src/lib_tests.rs",
        None,
    );
}

#[test]
fn reports_missing_path_bridge_even_when_sidecar_exists() {
    let results = assertions::check(&input(
        vec![
            file(
                "src/lib.rs",
                G3RsTestFileKind::Source,
                Some(""),
                Some("lib"),
                None,
                "#[cfg(test)]\nmod lib_tests;\n",
            ),
            file(
                "src/lib_tests/mod.rs",
                G3RsTestFileKind::InternalSidecarMod,
                Some(""),
                Some("lib"),
                None,
                "#[test]\nfn keeps_shape() { assert!(true); }\n",
            ),
        ],
        vec![component("", "", Some("demo_runtime"), false, None)],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-02",
        G3Severity::Error,
        "ad hoc cfg(test) module declaration",
        "src/lib.rs",
        Some(1),
    );

    assertions::assert_message(
        &results,
        "RS-TEST-FILETREE-02",
        "ad hoc cfg(test) module declaration",
        "src/lib.rs",
        "File `src/lib.rs` declares `#[cfg(test)] mod lib_tests;` without the owned sidecar declaration `#[path = \"lib_tests/mod.rs\"] mod lib_tests;`. Use that exact file-owned sidecar shape, so this module's internal tests live in one sidecar directory."
    );
}

#[test]
fn reports_generic_tests_name_even_with_owned_sidecar_folder() {
    let results = assertions::check(&input(
        vec![
            file(
                "src/rule.rs",
                G3RsTestFileKind::Source,
                Some(""),
                Some("rule"),
                None,
                "#[cfg(test)]\n#[path = \"rule_tests/mod.rs\"]\nmod tests;\n",
            ),
            file(
                "src/rule_tests/mod.rs",
                G3RsTestFileKind::InternalSidecarMod,
                Some(""),
                Some("rule"),
                None,
                "#[test]\nfn keeps_shape() { assert!(true); }\n",
            ),
        ],
        vec![component("", "", Some("demo_runtime"), false, None)],
    ));

    assertions::assert_message(
        &results,
        "RS-TEST-FILETREE-02",
        "ad hoc cfg(test) module declaration",
        "src/rule.rs",
        "File `src/rule.rs` declares `#[cfg(test)] mod tests;` without the owned sidecar declaration `#[path = \"rule_tests/mod.rs\"] mod rule_tests;`. Use that exact file-owned sidecar shape, so this module's internal tests live in one sidecar directory."
    );
}

#[test]
fn reports_orphaned_sidecar_harness() {
    let results = assertions::check(&input(
        vec![file(
            "src/foo_tests/mod.rs",
            G3RsTestFileKind::InternalSidecarMod,
            Some(""),
            Some("foo"),
            None,
            "#[test]\nfn stray() { assert!(true); }\n",
        )],
        vec![component("", "", Some("demo_runtime"), false, None)],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-02",
        G3Severity::Error,
        "orphaned sidecar harness",
        "src/foo_tests/mod.rs",
        None,
    );
}

#[test]
fn reports_sidecar_directory_missing_mod_rs() {
    let results = assertions::check(&input(
        vec![
            file(
                "src/foo.rs",
                G3RsTestFileKind::Source,
                Some(""),
                Some("foo"),
                None,
                "pub fn value() -> u8 { 1 }\n",
            ),
            file(
                "src/foo_tests/helper.rs",
                G3RsTestFileKind::InternalSidecarSupport,
                Some(""),
                Some("foo"),
                None,
                "#[test]\nfn stray() { assert!(true); }\n",
            ),
        ],
        vec![component("", "", Some("demo_runtime"), false, None)],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-02",
        G3Severity::Error,
        "sidecar directory missing mod.rs",
        "src/foo_tests",
        None,
    );
}

fn file(
    rel_path: &str,
    kind: G3RsTestFileKind,
    component_rel_dir: Option<&str>,
    owner_module_name: Option<&str>,
    assertions_package_name: Option<&str>,
    content: &str,
) -> g3rs_test_types::G3RsTestSourceFile {
    g3rs_test_types::G3RsTestSourceFile {
        rel_path: rel_path.to_owned(),
        kind,
        owner_module_name: owner_module_name.map(str::to_owned),
        component_rel_dir: component_rel_dir.map(str::to_owned),
        assertions_package_name: assertions_package_name.map(str::to_owned),
        content: content.to_owned(),
    }
}

fn component(
    rel_dir: &str,
    runtime_rel_dir: &str,
    runtime_package_name: Option<&str>,
    assertions_exists: bool,
    assertions_package_name: Option<&str>,
) -> g3rs_test_types::G3RsTestComponentFileTreeFacts {
    g3rs_test_types::G3RsTestComponentFileTreeFacts {
        rel_dir: rel_dir.to_owned(),
        runtime_rel_dir: runtime_rel_dir.to_owned(),
        runtime_cargo_rel_path: if runtime_rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{runtime_rel_dir}/Cargo.toml")
        },
        runtime_package_name: runtime_package_name.map(str::to_owned),
        runtime_normal_dependencies: BTreeSet::new(),
        runtime_dev_dependencies: BTreeSet::new(),
        assertions_rel_dir: if runtime_rel_dir.is_empty() {
            "assertions".to_owned()
        } else {
            format!("{}/assertions", parent_dir(runtime_rel_dir))
        },
        assertions_cargo_rel_path: if runtime_rel_dir.is_empty() {
            "assertions/Cargo.toml".to_owned()
        } else {
            format!("{}/assertions/Cargo.toml", parent_dir(runtime_rel_dir))
        },
        assertions_exists,
        nested_assertions_cargo_rel_path: None,
        assertions_package_name: assertions_package_name.map(str::to_owned),
        assertions_dependencies: BTreeSet::new(),
        sidecars: Vec::new(),
        external_harnesses: Vec::new(),
    }
}

fn input(
    files: Vec<g3rs_test_types::G3RsTestSourceFile>,
    components: Vec<g3rs_test_types::G3RsTestComponentFileTreeFacts>,
) -> g3rs_test_types::G3RsTestFileTreeChecksInput {
    let local_package_names = components
        .iter()
        .filter_map(|component| component.runtime_package_name.clone())
        .chain(
            components
                .iter()
                .filter_map(|component| component.assertions_package_name.clone()),
        )
        .collect();
    g3rs_test_types::G3RsTestFileTreeChecksInput {
        root_rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        files,
        components,
        local_package_names,
        input_failures: Vec::new(),
    }
}

fn parent_dir(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}
