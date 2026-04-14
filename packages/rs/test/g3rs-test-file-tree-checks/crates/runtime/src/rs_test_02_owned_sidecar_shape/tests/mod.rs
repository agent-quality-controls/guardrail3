use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use crate::test_helpers::{
    assert_has_inventory, assert_has_result, component, file, input, run_input,
};

#[test]
fn reports_inventory_for_owned_sidecar_shape() {
    let results = run_input(input(
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

    assert_has_inventory(
        &results,
        "RS-TEST-FILETREE-02",
        "owned sidecar shape confirmed",
        "Cargo.toml",
    );
}

#[test]
fn reports_ad_hoc_src_tests_tree() {
    let results = run_input(input(
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

    assert_has_result(
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
    let results = run_input(input(
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

    assert_has_result(
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
    let results = run_input(input(
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

    assert_has_result(
        &results,
        "RS-TEST-FILETREE-02",
        G3Severity::Error,
        "flat sidecar test file",
        "src/lib_tests.rs",
        None,
    );
}

#[test]
fn reports_bad_cfg_test_declaration() {
    let results = run_input(input(
        vec![file(
            "src/lib.rs",
            G3RsTestFileKind::Source,
            Some(""),
            Some("lib"),
            None,
            "#[cfg(test)]\nmod helper_tests;\n",
        )],
        vec![component("", "", Some("demo_runtime"), false, None)],
    ));

    assert_has_result(
        &results,
        "RS-TEST-FILETREE-02",
        G3Severity::Error,
        "ad hoc cfg(test) module declaration",
        "src/lib.rs",
        Some(1),
    );

    let result = results
        .iter()
        .find(|result| {
            result.id() == "RS-TEST-FILETREE-02"
                && result.title() == "ad hoc cfg(test) module declaration"
                && result.file() == Some("src/lib.rs")
        })
        .expect("missing RS-TEST-FILETREE-02 result");
    assert_eq!(
        result.message(),
        "File `src/lib.rs` declares `#[cfg(test)] mod helper_tests;` without the owned sidecar path `helper_tests/mod.rs`. Point that declaration at `helper_tests/mod.rs`, so this module's internal tests live in one sidecar directory."
    );
}

#[test]
fn reports_orphaned_sidecar_harness() {
    let results = run_input(input(
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

    assert_has_result(
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
    let results = run_input(input(
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

    assert_has_result(
        &results,
        "RS-TEST-FILETREE-02",
        G3Severity::Error,
        "sidecar directory missing mod.rs",
        "src/foo_tests",
        None,
    );
}
