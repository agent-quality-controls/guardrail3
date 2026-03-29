use super::{CargoFixture, cargo_fixture, check_results, entry, tree};
use guardrail3_app_rs_family_arch_assertions::rs_arch_06_owner_family_enablement_coherence as assertions;

#[test]
fn app_roots_error_when_effective_hexarch_enablement_is_false() {
    let config = "[rust.checks]\narch = true\nhexarch = false\nlibarch = true\n";
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
                cargo_fixture(CargoFixture::AppWorkspace),
            ),
        ],
    ));

    assertions::assert_error_files(&results, "RS-ARCH-06", &["apps/backend/Cargo.toml"]);
}
