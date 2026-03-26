use super::super::super::test_support::{assert_error_files, check_results, entry, tree};

#[test]
fn app_scoped_arch_config_is_forbidden() {
    let config = "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n\n[rust.apps.backend.checks]\narch = false\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &["guardrail3.toml"])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = []\nresolver = \"2\"\n",
            ),
        ],
    ));

    assert_error_files(&results, "RS-ARCH-05", &["guardrail3.toml"]);
}

#[test]
fn package_scoped_arch_config_is_forbidden() {
    let config = "[rust.checks]\narch = true\nhexarch = true\nlibarch = true\n\n[rust.packages.checks]\narch = false\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["packages"], &["guardrail3.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            (
                "packages/shared/Cargo.toml",
                "[package]\nname = \"shared\"\n",
            ),
        ],
    ));

    assert_error_files(&results, "RS-ARCH-05", &["guardrail3.toml"]);
}
