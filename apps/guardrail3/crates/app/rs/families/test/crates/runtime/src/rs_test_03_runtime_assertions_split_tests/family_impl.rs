#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_03_runtime_assertions_split::{assert_reported, assert_rule_files, assert_rule_quiet};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};

#[test]
fn root_local_ruleish_shape_still_requires_runtime_assertions_split() {let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "src/lib.rs",
        "#[cfg(test)]\nmod test_support;\nmod rs_demo_01;\n",
    );
    write_file(root, "src/test_support.rs", "pub fn helper() {}\n");
    write_file(
        root,
        "src/rs_demo_01.rs",
        "#[cfg(test)]\n#[path = \"rs_demo_01_tests/mod.rs\"]\nmod tests;\npub fn check() -> bool {true}\n",
    );
    write_file(
        root,
        "src/rs_demo_01_tests/mod.rs",
        "#[test]\nfn sidecar() {assert!(crate::rs_demo_01::check());}\n",
    );

    assert_rule_files(
        &run_family(root),
        vec!["src/rs_demo_01_tests/mod.rs".to_owned()],
    );}
