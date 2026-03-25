use super::super::super::test_support::{copy_fixture, run_family};

#[test]
fn golden_fixture_has_no_boundary_config_hits() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path());
    let hits: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-15")
        .collect();

    assert!(
        hits.is_empty(),
        "the golden fixture already has boundary configs for every app boundary: {hits:#?}"
    );
}
