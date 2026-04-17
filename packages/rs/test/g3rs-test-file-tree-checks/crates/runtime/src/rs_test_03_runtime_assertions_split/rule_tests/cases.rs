use std::collections::BTreeSet;

use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_file_tree_checks_assertions::rs_test_03_runtime_assertions_split::rule as assertions;

#[test]
fn reports_inventory_for_valid_runtime_assertions_split() {
    let component = with_external_harness(
        with_sidecar(
            {
                let mut component = component(
                    "",
                    "crates/runtime",
                    Some("demo_runtime"),
                    true,
                    Some("demo_assertions"),
                );
                component.runtime_dev_dependencies = BTreeSet::from(["demo_assertions".to_owned()]);
                component.assertions_dependencies = BTreeSet::from(["demo_runtime".to_owned()]);
                component
            },
            "crates/runtime/src/lib_tests/mod.rs",
            "crates/assertions/src/lib.rs",
        ),
        "crates/runtime/tests/public_surface.rs",
    );

    let results = assertions::check(&input(
        vec![
            file(
                "crates/runtime/src/lib.rs",
                G3RsTestFileKind::Source,
                Some(""),
                Some("lib"),
                Some("demo_assertions"),
                "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod lib_tests;\n\npub fn value() -> u8 { 1 }\n",
            ),
            file(
                "crates/runtime/src/lib_tests/mod.rs",
                G3RsTestFileKind::InternalSidecarMod,
                Some(""),
                Some("lib"),
                Some("demo_assertions"),
                "use demo_assertions::assert_runtime;\n#[test]\nfn owned_sidecar() { assert_runtime(); }\n",
            ),
            file(
                "crates/runtime/tests/public_surface.rs",
                G3RsTestFileKind::ExternalHarness,
                Some(""),
                None,
                Some("demo_assertions"),
                "use demo_assertions::assert_runtime;\n#[test]\nfn public_surface() { assert_runtime(); }\n",
            ),
            file(
                "crates/assertions/src/lib.rs",
                G3RsTestFileKind::AssertionsModule,
                Some(""),
                Some("lib"),
                Some("demo_assertions"),
                "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
            ),
        ],
        vec![component],
    ));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-FILETREE-03",
        "runtime/assertions split confirmed",
        "Cargo.toml",
    );
}

#[test]
fn reports_missing_assertions_crate() {
    let component = with_external_harness(
        component("", "crates/runtime", Some("demo_runtime"), false, None),
        "crates/runtime/tests/public_surface.rs",
    );
    let results = assertions::check(&input(
        vec![file(
            "crates/runtime/tests/public_surface.rs",
            G3RsTestFileKind::ExternalHarness,
            Some(""),
            None,
            None,
            "#[test]\nfn public_surface() { assert!(true); }\n",
        )],
        vec![component],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "assertions crate missing",
        "crates/assertions/Cargo.toml",
        None,
    );

    assertions::assert_message(
        &results,
        "RS-TEST-FILETREE-03",
        "assertions crate missing",
        "crates/assertions/Cargo.toml",
        "Component `crates/runtime` has sidecar tests that require a shared assertions crate, but `crates/runtime` is still a single-crate package. Reshape it into one package with sibling member crates under `crates/`: `crates/runtime` for the production crate and `crates/assertions` for shared test proof. Do not add `crates/runtime/assertions/Cargo.toml` directly under the current crate root, because that creates a nested package instead of sibling member crates."
    );
}

#[test]
fn reports_nested_assertions_package_as_wrong_shape() {
    let component = with_nested_assertions_manifest(
        with_external_harness(
            component(
                "packages/demo",
                "packages/demo/crates/runtime",
                Some("demo_runtime"),
                false,
                None,
            ),
            "packages/demo/crates/runtime/tests/public_surface.rs",
        ),
        "packages/demo/assertions/Cargo.toml",
    );
    let results = assertions::check(&input(
        vec![file(
            "packages/demo/crates/runtime/tests/public_surface.rs",
            G3RsTestFileKind::ExternalHarness,
            Some("packages/demo"),
            None,
            None,
            "#[test]\nfn public_surface() { assert!(true); }\n",
        )],
        vec![component],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "nested assertions package is the wrong shape",
        "packages/demo/assertions/Cargo.toml",
        None,
    );

    assertions::assert_message(
        &results,
        "RS-TEST-FILETREE-03",
        "nested assertions package is the wrong shape",
        "packages/demo/assertions/Cargo.toml",
        "Found nested package `packages/demo/assertions/Cargo.toml`. This is the wrong test layout. If assertions is a separate crate, move it to `packages/demo/crates/assertions/Cargo.toml` and move the production crate to `packages/demo/crates/runtime/Cargo.toml` so both are sibling member crates in one package."
    );
}

#[test]
fn reports_runtime_depends_on_assertions_at_normal_scope() {
    let mut component = with_external_harness(
        component(
            "",
            "crates/runtime",
            Some("demo_runtime"),
            true,
            Some("demo_assertions"),
        ),
        "crates/runtime/tests/public_surface.rs",
    );
    component.runtime_normal_dependencies = BTreeSet::from(["demo_assertions".to_owned()]);
    component.runtime_dev_dependencies = BTreeSet::from(["demo_assertions".to_owned()]);
    component.assertions_dependencies = BTreeSet::from(["demo_runtime".to_owned()]);

    let results = assertions::check(&input(
        vec![
            file(
                "crates/runtime/tests/public_surface.rs",
                G3RsTestFileKind::ExternalHarness,
                Some(""),
                None,
                Some("demo_assertions"),
                "use demo_assertions::assert_runtime;\n#[test]\nfn public_surface() { assert_runtime(); }\n",
            ),
            file(
                "crates/assertions/src/lib.rs",
                G3RsTestFileKind::AssertionsModule,
                Some(""),
                Some("lib"),
                Some("demo_assertions"),
                "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
            ),
        ],
        vec![component],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "runtime depends on assertions at normal scope",
        "crates/runtime/Cargo.toml",
        None,
    );
}

#[test]
fn reports_assertions_missing_runtime_dependency() {
    let mut component = with_external_harness(
        component(
            "",
            "crates/runtime",
            Some("demo_runtime"),
            true,
            Some("demo_assertions"),
        ),
        "crates/runtime/tests/public_surface.rs",
    );
    component.runtime_dev_dependencies = BTreeSet::from(["demo_assertions".to_owned()]);

    let results = assertions::check(&input(
        vec![
            file(
                "crates/runtime/tests/public_surface.rs",
                G3RsTestFileKind::ExternalHarness,
                Some(""),
                None,
                Some("demo_assertions"),
                "use demo_assertions::assert_runtime;\n#[test]\nfn public_surface() { assert_runtime(); }\n",
            ),
            file(
                "crates/assertions/src/lib.rs",
                G3RsTestFileKind::AssertionsModule,
                Some(""),
                Some("lib"),
                Some("demo_assertions"),
                "pub fn assert_runtime() {}\n",
            ),
        ],
        vec![component],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "assertions missing runtime dependency",
        "crates/assertions/Cargo.toml",
        None,
    );

    assertions::assert_message(
        &results,
        "RS-TEST-FILETREE-03",
        "assertions missing runtime dependency",
        "crates/assertions/Cargo.toml",
        "Manifest `crates/assertions/Cargo.toml` is missing dependency `demo_runtime`. Add `demo_runtime` under `[dependencies]`, so the shared assertions crate can prove the runtime behavior it checks."
    );
}

#[test]
fn reports_external_harness_reaching_private_runtime_glue() {
    let component = with_external_harness(
        component(
            "",
            "crates/runtime",
            Some("demo_runtime"),
            true,
            Some("demo_assertions"),
        ),
        "crates/runtime/tests/public_surface.rs",
    );
    let results = assertions::check(&input(
        vec![file(
            "crates/runtime/tests/public_surface.rs",
            G3RsTestFileKind::ExternalHarness,
            Some(""),
            None,
            Some("demo_assertions"),
            "use crate::value;\n#[test]\nfn public_surface() { assert_eq!(value(), 1); }\n",
        )],
        vec![component],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "external harness reaches private runtime glue",
        "crates/runtime/tests/public_surface.rs",
        Some(1),
    );
}

#[test]
fn reports_external_harness_path_including_local_source() {
    let component = with_external_harness(
        component(
            "",
            "crates/runtime",
            Some("demo_runtime"),
            true,
            Some("demo_assertions"),
        ),
        "crates/runtime/tests/public_surface.rs",
    );
    let results = assertions::check(&input(
        vec![file(
            "crates/runtime/tests/public_surface.rs",
            G3RsTestFileKind::ExternalHarness,
            Some(""),
            None,
            Some("demo_assertions"),
            "#[path = \"../src/lib.rs\"]\nmod runtime_source;\n#[test]\nfn public_surface() { assert_eq!(runtime_source::value(), 1); }\n",
        )],
        vec![component],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "external harness path-includes local source",
        "crates/runtime/tests/public_surface.rs",
        Some(1),
    );
}

#[test]
fn reports_sidecar_importing_sibling_production_module() {
    let component = with_sidecar(
        {
            let mut component = component(
                "",
                "crates/runtime",
                Some("demo_runtime"),
                true,
                Some("demo_assertions"),
            );
            component.runtime_dev_dependencies = BTreeSet::from(["demo_assertions".to_owned()]);
            component.assertions_dependencies = BTreeSet::from(["demo_runtime".to_owned()]);
            component
        },
        "crates/runtime/src/foo_tests/mod.rs",
        "crates/assertions/src/foo.rs",
    );
    let results = assertions::check(&input(
        vec![
            file(
                "crates/runtime/src/foo.rs",
                G3RsTestFileKind::Source,
                Some(""),
                Some("foo"),
                Some("demo_assertions"),
                "pub fn value() -> u8 { 1 }\n",
            ),
            file(
                "crates/runtime/src/bar.rs",
                G3RsTestFileKind::Source,
                Some(""),
                Some("bar"),
                Some("demo_assertions"),
                "pub fn other() -> u8 { 2 }\n",
            ),
            file(
                "crates/runtime/src/foo_tests/mod.rs",
                G3RsTestFileKind::InternalSidecarMod,
                Some(""),
                Some("foo"),
                Some("demo_assertions"),
                "use crate::bar::other;\n#[test]\nfn owned_sidecar() { assert_eq!(other(), 2); }\n",
            ),
            file(
                "crates/assertions/src/foo.rs",
                G3RsTestFileKind::AssertionsModule,
                Some(""),
                Some("foo"),
                Some("demo_assertions"),
                "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
            ),
        ],
        vec![component],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "sidecar imports sibling local module",
        "crates/runtime/src/foo_tests/mod.rs",
        Some(1),
    );

    assertions::assert_message(
        &results,
        "RS-TEST-FILETREE-03",
        "sidecar imports sibling local module",
        "crates/runtime/src/foo_tests/mod.rs",
        "Sidecar file `crates/runtime/src/foo_tests/mod.rs` imports sibling local module `bar`. Import only the owned production module `foo` or the shared assertions crate from this sidecar, so the sidecar tests one module without reaching into siblings."
    );
}

#[test]
fn allows_nested_sidecar_to_import_its_required_nested_assertions_module() {
    let component = with_sidecar(
        {
            let mut component = component(
                "",
                "crates/runtime",
                Some("demo_runtime"),
                true,
                Some("demo_assertions"),
            );
            component.runtime_dev_dependencies = BTreeSet::from(["demo_assertions".to_owned()]);
            component.assertions_dependencies = BTreeSet::from(["demo_runtime".to_owned()]);
            component
        },
        "crates/runtime/src/foo/rule_tests/mod.rs",
        "crates/assertions/src/foo/rule.rs",
    );

    let results = assertions::check(&input(
        vec![
            file(
                "crates/runtime/src/foo/rule.rs",
                G3RsTestFileKind::Source,
                Some(""),
                Some("rule"),
                Some("demo_assertions"),
                "pub fn value() -> u8 { 1 }\n",
            ),
            file(
                "crates/runtime/src/foo/rule_tests/mod.rs",
                G3RsTestFileKind::InternalSidecarMod,
                Some(""),
                Some("rule"),
                Some("demo_assertions"),
                "mod golden;\n",
            ),
            file(
                "crates/runtime/src/foo/rule_tests/golden.rs",
                G3RsTestFileKind::InternalSidecarSupport,
                Some(""),
                Some("rule"),
                Some("demo_assertions"),
                "use demo_assertions::foo::rule::assert_runtime;\n#[test]\nfn owned_sidecar() { assert_runtime(); }\n",
            ),
            file(
                "crates/assertions/src/foo/rule.rs",
                G3RsTestFileKind::AssertionsModule,
                Some(""),
                Some("rule"),
                Some("demo_assertions"),
                "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
            ),
        ],
        vec![component],
    ));

    assertions::assert_no_title(
        &results,
        "RS-TEST-FILETREE-03",
        "sidecar imports sibling assertions module",
    );
}

#[test]
fn reports_test_harness_outside_runtime_assertions_split() {
    let results = assertions::check(&input(
        vec![file(
            "src/lib_tests/mod.rs",
            G3RsTestFileKind::InternalSidecarMod,
            None,
            Some("lib"),
            None,
            "#[test]\nfn owned_sidecar() { assert!(true); }\n",
        )],
        vec![component(
            "",
            "crates/runtime",
            Some("demo_runtime"),
            false,
            None,
        )],
    ));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "test harness outside runtime/assertions split",
        "src/lib_tests/mod.rs",
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

fn with_nested_assertions_manifest(
    mut component: g3rs_test_types::G3RsTestComponentFileTreeFacts,
    nested_assertions_cargo_rel_path: &str,
) -> g3rs_test_types::G3RsTestComponentFileTreeFacts {
    component.nested_assertions_cargo_rel_path = Some(nested_assertions_cargo_rel_path.to_owned());
    component
}

fn with_sidecar(
    mut component: g3rs_test_types::G3RsTestComponentFileTreeFacts,
    mod_rel_path: &str,
    assertions_module_rel_path: &str,
) -> g3rs_test_types::G3RsTestComponentFileTreeFacts {
    component
        .sidecars
        .push(g3rs_test_types::G3RsTestOwnedSidecarFacts {
            mod_rel_path: mod_rel_path.to_owned(),
            assertions_module_rel_path: assertions_module_rel_path.to_owned(),
        });
    component
}

fn with_external_harness(
    mut component: g3rs_test_types::G3RsTestComponentFileTreeFacts,
    rel_path: &str,
) -> g3rs_test_types::G3RsTestComponentFileTreeFacts {
    component.external_harnesses.push(rel_path.to_owned());
    component
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
