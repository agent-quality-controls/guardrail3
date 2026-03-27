use guardrail3_app_rs_family_code_assertions::rs_code_18_always_true_cfg_attr_bypass::{assert_no_hits};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn skips_genuinely_conditional_cfg_attr_forms() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let worker_rel = "apps/worker/crates/app/processor/src/lib.rs";
    let backend_content =
        test_support::read_file(root, backend_rel);
    let worker_content =
        test_support::read_file(root, worker_rel);

    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\n#[cfg_attr(test, allow(clippy::unwrap_used))]\nfn test_only_probe() {{}}\n#[cfg_attr(feature = \"serde\", allow(clippy::expect_used))]\nfn feature_probe() {{}}\n#[cfg_attr(any(test, feature = \"serde\"), allow(clippy::panic))]\nfn mixed_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        worker_rel,
        &format!(
            "{worker_content}\nstruct ImplProbe;\nimpl ImplProbe {{\n    #[cfg_attr(feature = \"debug-tools\", allow(clippy::too_many_lines))]\n    fn method_probe(&self) {{}}\n}}\n"
        ),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}
