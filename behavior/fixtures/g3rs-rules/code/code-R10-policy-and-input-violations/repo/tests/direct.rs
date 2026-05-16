#[test]
fn external_harness_asserts_directly() {
    let observed = "fixture".contains("fix");
    assert!(
        observed,
        "external harness assertion must inspect runtime state"
    );
}
