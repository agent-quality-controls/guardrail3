use crate::unknown_keys::check;
use g3rs_clippy_config_checks_assertions::unknown_keys as assertions;
use test_support::input_from_raw;

#[test]
fn warns_on_managed_key_typos() {
    let input = input_from_raw("clippy.toml", "too-many-lnes-threshold = 75\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        assertions::findings(&results),
        vec![assertions::warn(
            "unrecognized clippy.toml key",
            "Top-level key `too-many-lnes-threshold` looks like a typo of a guardrail-managed clippy key. Check the spelling and correct it.",
            "clippy.toml",
            false,
        )]
    );
}

#[test]
fn stays_quiet_for_benign_near_miss_keys() {
    let input = input_from_raw("clippy.toml", "allow-print-output-in-tests = true\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "no suspicious managed-key typos",
            "No top-level keys look like typos of guardrail-managed clippy keys.",
            "clippy.toml",
            true,
        )],
    );
}
