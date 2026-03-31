use super::{check_results, entry, tree};
use guardrail3_app_rs_family_topology_assertions::rs_topology_02_no_misplaced_roots as assertions;

#[test]
fn misplaced_roots_fire_when_hexarch_is_enabled() {
    let config = "[rust.checks]\nhexarch = true\nlibarch = false\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &["guardrail3.toml"])),
            ("tools", entry(&["worker"], &[])),
            ("tools/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            ("tools/worker/Cargo.toml", "[package]\nname = \"worker\"\n"),
        ],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-02", &["tools/worker/Cargo.toml"]);
}

#[test]
fn misplaced_roots_fire_when_libarch_is_enabled() {
    let config = "[rust.checks]\nhexarch = false\nlibarch = true\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &["guardrail3.toml"])),
            ("tools", entry(&["worker"], &[])),
            ("tools/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            ("tools/worker/Cargo.toml", "[package]\nname = \"worker\"\n"),
        ],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-02", &["tools/worker/Cargo.toml"]);
}

#[test]
fn misplaced_roots_do_not_fire_when_both_architecture_families_are_disabled() {
    let config = "[rust.checks]\nhexarch = false\nlibarch = false\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &["guardrail3.toml"])),
            ("tools", entry(&["worker"], &[])),
            ("tools/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            ("tools/worker/Cargo.toml", "[package]\nname = \"worker\"\n"),
        ],
    ));

    assertions::assert_no_error_files(&results, "RS-TOPOLOGY-02");
    assertions::assert_inventory_summary(
        &results,
        "RS-TOPOLOGY-02",
        "Misplaced-root reporting is inactive",
    );
}

#[test]
fn misplaced_roots_still_fire_when_arch_is_globally_disabled() {
    let config = "[rust.checks]\ntopology = false\nhexarch = true\nlibarch = true\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &["guardrail3.toml"])),
            ("tools", entry(&["worker"], &[])),
            ("tools/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            ("tools/worker/Cargo.toml", "[package]\nname = \"worker\"\n"),
        ],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-02", &["tools/worker/Cargo.toml"]);
}
