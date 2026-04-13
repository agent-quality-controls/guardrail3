use super::helpers::run_check;

#[test]
fn stays_quiet_when_specific_target_lints_are_absent() {
    let results = run_check(
        r#"
[workspace]
members = []

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
nursery = { level = "deny", priority = -1 }
cargo = { level = "deny", priority = -1 }
"#,
    );

    assert!(
        results
            .iter()
            .all(|result| result.id() != "RS-CARGO-CONFIG-04"),
        "{results:#?}"
    );
}
