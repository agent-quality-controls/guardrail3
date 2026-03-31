use guardrail3_app_rs_family_arch_assertions::rs_arch_16_workspace_local_file_placement as assertions;

use super::{check_results, entry, tree};

#[test]
fn reports_nested_cargo_sidecar_under_workspace_member() {
    let tree = tree(
        &[
            ("", entry(&["apps"], &["guardrail3.toml"])),
            ("apps", entry(&["api"], &[])),
            ("apps/api", entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", entry(&["member"], &[])),
            (
                "apps/api/crates/member",
                entry(&[".cargo"], &["Cargo.toml"]),
            ),
            ("apps/api/crates/member/.cargo", entry(&[], &["config.toml"])),
        ],
        &[
            ("guardrail3.toml", "[rust.checks]\narch = true\nclippy = true\n"),
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = [\"crates/member\"]\nresolver = \"2\"\n",
            ),
            (
                "apps/api/crates/member/Cargo.toml",
                "[package]\nname = \"member\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
            ),
            (
                "apps/api/crates/member/.cargo/config.toml",
                "[env]\nCLIPPY_CONF_DIR = { value = \"cfg\" }\n",
            ),
        ],
    );

    let results = check_results(&tree);

    assertions::assert_error_files(&results, &["apps/api/crates/member/.cargo/config.toml"]);
}
