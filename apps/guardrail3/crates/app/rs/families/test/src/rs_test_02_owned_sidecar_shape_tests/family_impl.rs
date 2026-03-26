use crate::test_support::{run_family, rule_files, tempdir, write_file};

#[test]
fn guardrail_family_rule_sidecars_and_test_support_stay_quiet() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, "src/lib.rs", "#[cfg(test)]\nmod test_support;\nmod rs_demo_01;\n");
    write_file(root, "src/test_support.rs", "pub fn helper() {}\n");
    write_file(
        root,
        "src/rs_demo_01.rs",
        "#[cfg(test)]\n#[path = \"rs_demo_01_tests/mod.rs\"]\nmod tests;\npub fn check() -> bool { true }\n",
    );
    write_file(
        root,
        "src/rs_demo_01_tests/mod.rs",
        "#[test]\nfn sidecar() { assert!(crate::rs_demo_01::check()); }\n",
    );

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-02").is_empty());
}
