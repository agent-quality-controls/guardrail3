use g3rs_release_source_checks_assertions::run as assertions;

use super::helpers;

#[test]
fn aggregates_quality_and_input_failures() {
    let results = super::super::check(&helpers::input());

    assertions::assert_result_ids(
        &results,
        &[
            "g3rs-release/source-input-failures",
            "g3rs-release/readme-quality",
        ],
    );
}
