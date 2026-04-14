use crate::rs_clippy_config_17_unknown_keys::check;
use crate::test_support::{findings, input_from_raw};

#[test]
fn warns_on_managed_key_typos() {
    let input = input_from_raw("clippy.toml", "too-many-lnes-threshold = 75\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        findings(&results),
        vec![crate::test_support::Finding {
            id: "RS-CLIPPY-CONFIG-17".to_owned(),
            severity: guardrail3_check_types::G3Severity::Warn,
            title: "unrecognized clippy.toml key".to_owned(),
            message: "Top-level key `too-many-lnes-threshold` looks like a typo of a guardrail-managed clippy key. Check the spelling and correct it.".to_owned(),
            file: Some("clippy.toml".to_owned()),
            inventory: false,
        }]
    );
}

#[test]
fn stays_quiet_for_benign_near_miss_keys() {
    let input = input_from_raw("clippy.toml", "allow-print-output-in-tests = true\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "no suspicious managed-key typos" && finding.inventory
    }));
}
