use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_22_ports_trait_dominance as assertions;

#[test]
fn unparsable_ports_source_warns_in_family_run() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/ports/outbound/repo/src/lib.rs"),
        "pub trait Repo {\n",
    )
    .expect("write broken ports source");

    let results = super::run_family(tmp.path());
    assertions::assert_warning_summary(
        &results,
        "",
        1,
        &["apps/backend/crates/ports/outbound/repo/src/lib.rs"],
        Some(Some("apps/backend/crates/ports/outbound/repo/src/lib.rs")),
        Some("Failed to parse Rust source file"),
        None,
        &[],
    );
}

#[test]
fn parse_failure_takes_precedence_over_impl_heavy_warning() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/ports/outbound/repo/src/lib.rs"),
        "mod extra;\npub trait Repo {\n",
    )
    .expect("write broken ports source");
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/ports/outbound/repo/src/extra.rs"),
        "pub struct ExtraA;\nimpl ExtraA { pub fn new() -> Self { Self } }\npub struct ExtraB;\nimpl ExtraB { pub fn new() -> Self { Self } }\n",
    )
    .expect("write impl-heavy extra module");

    let results = super::run_family(tmp.path());
    assertions::assert_warning_summary(
        &results,
        "",
        1,
        &["apps/backend/crates/ports/outbound/repo/src/lib.rs"],
        Some(Some("apps/backend/crates/ports/outbound/repo/src/lib.rs")),
        Some("Failed to parse Rust source file"),
        None,
        &["impl-heavy"],
    );
}
