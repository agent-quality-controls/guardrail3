use std::collections::BTreeSet;

use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use crate::test_helpers::{
    assert_has_inventory, assert_has_result, component, file, input, run_input,
    with_external_harness, with_nested_assertions_manifest, with_sidecar,
};

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

    let results = run_input(input(
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

    assert_has_inventory(
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
    let results = run_input(input(
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

    assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "assertions crate missing",
        "crates/assertions/Cargo.toml",
        None,
    );

    let result = results
        .iter()
        .find(|result| {
            result.id() == "RS-TEST-FILETREE-03"
                && result.title() == "assertions crate missing"
                && result.file() == Some("crates/assertions/Cargo.toml")
        })
        .expect("missing RS-TEST-FILETREE-03 result");
    assert_eq!(
        result.message(),
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
    let results = run_input(input(
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

    assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "nested assertions package is the wrong shape",
        "packages/demo/assertions/Cargo.toml",
        None,
    );

    let result = results
        .iter()
        .find(|result| {
            result.id() == "RS-TEST-FILETREE-03"
                && result.title() == "nested assertions package is the wrong shape"
                && result.file() == Some("packages/demo/assertions/Cargo.toml")
        })
        .expect("missing RS-TEST-FILETREE-03 result");
    assert_eq!(
        result.message(),
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

    let results = run_input(input(
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

    assert_has_result(
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

    let results = run_input(input(
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

    assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "assertions missing runtime dependency",
        "crates/assertions/Cargo.toml",
        None,
    );

    let result = results
        .iter()
        .find(|result| {
            result.id() == "RS-TEST-FILETREE-03"
                && result.title() == "assertions missing runtime dependency"
                && result.file() == Some("crates/assertions/Cargo.toml")
        })
        .expect("missing RS-TEST-FILETREE-03 result");
    assert_eq!(
        result.message(),
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
    let results = run_input(input(
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

    assert_has_result(
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
    let results = run_input(input(
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

    assert_has_result(
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
    let results = run_input(input(
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

    assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "sidecar imports sibling local module",
        "crates/runtime/src/foo_tests/mod.rs",
        Some(1),
    );

    let result = results
        .iter()
        .find(|result| {
            result.id() == "RS-TEST-FILETREE-03"
                && result.title() == "sidecar imports sibling local module"
                && result.file() == Some("crates/runtime/src/foo_tests/mod.rs")
        })
        .expect("missing RS-TEST-FILETREE-03 result");
    assert_eq!(
        result.message(),
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

    let results = run_input(input(
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

    assert!(
        results.iter().all(|result| {
            !(result.id() == "RS-TEST-FILETREE-03"
                && result.title() == "sidecar imports sibling assertions module")
        }),
        "unexpected sibling assertions error\nactual={results:#?}"
    );
}

#[test]
fn reports_test_harness_outside_runtime_assertions_split() {
    let results = run_input(input(
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

    assert_has_result(
        &results,
        "RS-TEST-FILETREE-03",
        G3Severity::Error,
        "test harness outside runtime/assertions split",
        "src/lib_tests/mod.rs",
        None,
    );
}
