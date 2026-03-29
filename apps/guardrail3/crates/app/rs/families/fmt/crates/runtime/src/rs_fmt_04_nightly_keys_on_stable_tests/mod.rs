use guardrail3_app_rs_family_fmt_assertions::rs_fmt_04_nightly_keys_on_stable as assertions;

use super::{TestToolchainState, run_check, run_family};

#[test]
fn reports_nightly_only_keys_on_stable_toolchain() {
    let results = run_check(TestToolchainState::Stable);

    assertions::assert_warn_for_key(&results, "group_imports", "rustfmt.toml");
}

#[test]
fn ignores_nightly_keys_when_toolchain_is_not_stable() {
    let results = run_check(TestToolchainState::Other);

    assertions::assert_no_findings(&results);
}

#[test]
fn fails_closed_when_nightly_keys_have_no_toolchain_file() {
    let fixture = tempfile::tempdir().expect("create tempdir");
    let root = fixture.path();

    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n\n[workspace.package]\nedition = \"2024\"\n",
    )
    .expect("write Cargo.toml");
    std::fs::write(
        root.join("rustfmt.toml"),
        "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\ngroup_imports = \"StdExternalCrate\"\n",
    )
    .expect("write rustfmt.toml");

    let results = run_family(root);

    assertions::assert_error(
        &results,
        "rust-toolchain.toml missing",
        "Nightly-only rustfmt settings require a root rust-toolchain.toml to prove the channel.",
    );
}

#[test]
fn fails_closed_when_toolchain_file_is_malformed() {
    let fixture = tempfile::tempdir().expect("create tempdir");
    let root = fixture.path();

    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n\n[workspace.package]\nedition = \"2024\"\n",
    )
    .expect("write Cargo.toml");
    std::fs::write(
        root.join("rustfmt.toml"),
        "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\ngroup_imports = \"StdExternalCrate\"\n",
    )
    .expect("write rustfmt.toml");
    std::fs::write(root.join("rust-toolchain.toml"), "[toolchain")
        .expect("write rust-toolchain.toml");

    let results = run_family(root);

    assertions::assert_error(
        &results,
        "rust-toolchain.toml parse error",
        "Nightly-only rustfmt settings require a parseable root rust-toolchain.toml.",
    );
}

#[test]
fn fails_closed_when_toolchain_channel_is_missing() {
    let fixture = tempfile::tempdir().expect("create tempdir");
    let root = fixture.path();

    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n\n[workspace.package]\nedition = \"2024\"\n",
    )
    .expect("write Cargo.toml");
    std::fs::write(
        root.join("rustfmt.toml"),
        "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\ngroup_imports = \"StdExternalCrate\"\n",
    )
    .expect("write rustfmt.toml");
    std::fs::write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\ncomponents = [\"rustfmt\"]\n",
    )
    .expect("write rust-toolchain.toml");

    let results = run_family(root);

    assertions::assert_error(
        &results,
        "rust-toolchain channel missing",
        "Nightly-only rustfmt settings require `[toolchain].channel` in root rust-toolchain.toml.",
    );
}
