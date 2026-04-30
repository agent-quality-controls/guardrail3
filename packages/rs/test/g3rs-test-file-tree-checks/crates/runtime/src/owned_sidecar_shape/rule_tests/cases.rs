use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_file_tree_checks_assertions::owned_sidecar_shape::rule as assertions;
use g3rs_test_ingestion_runtime::fixtures::file_tree::{component, file, input, with_sidecar};

#[test]
fn reports_inventory_for_owned_sidecar_shape() {
    let results = assertions::check(&input(
        vec![
            file(
                "src/run.rs",
                G3RsTestFileKind::Source,
                Some(""),
                Some("run"),
                None,
                "#[cfg(test)]\n#[path = \"run_tests/mod.rs\"]\nmod run_tests;\n",
            ),
            file(
                "src/run_tests/mod.rs",
                G3RsTestFileKind::InternalSidecarMod,
                Some(""),
                Some("run"),
                None,
                "#[test]\nfn keeps_shape() { assert!(true); }\n",
            ),
        ],
        vec![with_sidecar(
            component("", "", Some("demo_runtime"), false, None),
            "src/run_tests/mod.rs",
            "assertions/src/run.rs",
        )],
    ));

    assertions::assert_has_inventory(
        &results,
        "g3rs-test/owned-sidecar-shape",
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
        "g3rs-test/owned-sidecar-shape",
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
        "g3rs-test/owned-sidecar-shape",
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
        "g3rs-test/owned-sidecar-shape",
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
                "src/run.rs",
                G3RsTestFileKind::Source,
                Some(""),
                Some("run"),
                None,
                "#[cfg(test)]\nmod run_tests;\n",
            ),
            file(
                "src/run_tests/mod.rs",
                G3RsTestFileKind::InternalSidecarMod,
                Some(""),
                Some("run"),
                None,
                "#[test]\nfn keeps_shape() { assert!(true); }\n",
            ),
        ],
        vec![with_sidecar(
            component("", "", Some("demo_runtime"), false, None),
            "src/run_tests/mod.rs",
            "assertions/src/run.rs",
        )],
    ));

    assertions::assert_has_result(
        &results,
        "g3rs-test/owned-sidecar-shape",
        G3Severity::Error,
        "ad hoc cfg(test) module declaration",
        "src/run.rs",
        Some(1),
    );

    assertions::assert_message(
        &results,
        "g3rs-test/owned-sidecar-shape",
        "ad hoc cfg(test) module declaration",
        "src/run.rs",
        "File `src/run.rs` declares `#[cfg(test)] mod run_tests;` without the owned sidecar declaration `#[path = \"run_tests/mod.rs\"] mod run_tests;`. Use that exact file-owned sidecar shape, so this module's internal tests live in one sidecar directory.",
    );
}

#[test]
fn reports_facade_lib_owned_sidecar_even_with_exact_lib_tests_shape() {
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
        vec![with_sidecar(
            component("", "", Some("demo_runtime"), false, None),
            "src/lib_tests/mod.rs",
            "assertions/src/lib.rs",
        )],
    ));

    assertions::assert_has_result(
        &results,
        "g3rs-test/owned-sidecar-shape",
        G3Severity::Error,
        "ad hoc cfg(test) module declaration",
        "src/lib.rs",
        Some(1),
    );

    assertions::assert_message(
        &results,
        "g3rs-test/owned-sidecar-shape",
        "ad hoc cfg(test) module declaration",
        "src/lib.rs",
        "Facade file `src/lib.rs` must not declare internal test sidecars. Move the tests onto a real sibling `x.rs` file and use `#[path = \"x_tests/mod.rs\"] mod x_tests;` there.",
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
        vec![with_sidecar(
            component("", "", Some("demo_runtime"), false, None),
            "src/rule_tests/mod.rs",
            "assertions/src/rule.rs",
        )],
    ));

    assertions::assert_message(
        &results,
        "g3rs-test/owned-sidecar-shape",
        "ad hoc cfg(test) module declaration",
        "src/rule.rs",
        "File `src/rule.rs` declares `#[cfg(test)] mod tests;` without the owned sidecar declaration `#[path = \"rule_tests/mod.rs\"] mod rule_tests;`. Use that exact file-owned sidecar shape, so this module's internal tests live in one sidecar directory.",
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
        vec![with_sidecar(
            component("", "", Some("demo_runtime"), false, None),
            "src/foo_tests/mod.rs",
            "assertions/src/foo.rs",
        )],
    ));

    assertions::assert_has_result(
        &results,
        "g3rs-test/owned-sidecar-shape",
        G3Severity::Error,
        "orphaned sidecar harness",
        "src/foo_tests/mod.rs",
        None,
    );
}

#[test]
fn does_not_report_prebound_owned_sidecar_as_missing_when_file_bag_is_lossy() {
    let results = assertions::check(&input(
        vec![file(
            "src/foo.rs",
            G3RsTestFileKind::Source,
            Some(""),
            Some("foo"),
            None,
            "#[cfg(test)]\n#[path = \"foo_tests/mod.rs\"]\nmod foo_tests;\n",
        )],
        vec![with_sidecar(
            component("", "", Some("demo_runtime"), false, None),
            "src/foo_tests/mod.rs",
            "assertions/src/foo.rs",
        )],
    ));

    assertions::assert_no_title(
        &results,
        "g3rs-test/owned-sidecar-shape",
        "sidecar directory missing mod.rs",
    );
    assertions::assert_no_title(
        &results,
        "g3rs-test/owned-sidecar-shape",
        "orphaned sidecar harness",
    );
    assertions::assert_no_title(
        &results,
        "g3rs-test/owned-sidecar-shape",
        "ad hoc cfg(test) module declaration",
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
        "g3rs-test/owned-sidecar-shape",
        G3Severity::Error,
        "sidecar directory missing mod.rs",
        "src/foo_tests",
        None,
    );
}
