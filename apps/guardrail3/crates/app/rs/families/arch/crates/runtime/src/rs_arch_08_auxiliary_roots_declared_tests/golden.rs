use super::{check_results, entry, tree};
use guardrail3_app_rs_family_arch_assertions::rs_arch_08_auxiliary_roots_declared as assertions;

#[test]
fn no_auxiliary_info_results_when_no_auxiliary_roots_exist() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "apps/backend/Cargo.toml",
            "[workspace]\nmembers = []\nresolver = \"2\"\n",
        )],
    ));

    assertions::assert_no_info_files(&results, "RS-ARCH-08");
}
