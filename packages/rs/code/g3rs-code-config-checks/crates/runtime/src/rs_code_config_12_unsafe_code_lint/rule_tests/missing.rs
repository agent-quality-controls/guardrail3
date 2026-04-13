use super::helpers::{cargo_file, run_check};

#[test]
fn stays_clean_for_missing_or_other_levels() {
    let results = run_check(vec![
        cargo_file("Cargo.toml", "[workspace]\nmembers = []\n"),
        cargo_file(
            "nested/Cargo.toml",
            "[workspace]\n[workspace.lints.rust]\nunsafe_code = \"warn\"\n",
        ),
    ]);

    assert!(results.is_empty(), "unexpected results: {results:#?}");
}
