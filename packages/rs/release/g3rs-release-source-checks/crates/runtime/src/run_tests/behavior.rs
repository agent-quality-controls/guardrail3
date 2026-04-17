use g3rs_release_source_checks_assertions::run as assertions;

use super::helpers;

#[test]
fn aggregates_quality_and_input_failures() {
    let results = super::super::check(&helpers::input());

    assertions::assert_result_ids(
        &results,
        &["RS-RELEASE-SOURCE-02", "RS-RELEASE-SOURCE-01"],
    );
}
