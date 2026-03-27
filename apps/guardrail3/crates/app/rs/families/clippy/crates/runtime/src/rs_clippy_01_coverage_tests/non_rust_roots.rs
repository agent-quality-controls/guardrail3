use super::super::super::test_support::{
    canonical_clippy_toml, copy_fixture, run_family, write_file,
};

#[test]
fn ignores_non_rust_roots_in_the_multi_root_fixture() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "clippy.toml", &canonical_clippy_toml());

    let results = run_family(tmp.path());
    let coverage_messages = results
        .iter()
        .filter(|result| result.id == "RS-CLIPPY-01")
        .map(|result| result.message.as_str())
        .collect::<Vec<_>>();

    assert!(
        coverage_messages.iter().all(
            |message| !message.contains("apps/landing") && !message.contains("packages/ui-kit")
        ),
        "expected non-Rust roots to stay out of clippy coverage results: {coverage_messages:#?}"
    );
}
