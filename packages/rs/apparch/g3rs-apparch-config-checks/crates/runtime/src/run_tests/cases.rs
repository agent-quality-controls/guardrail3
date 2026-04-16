use g3rs_apparch_config_checks_assertions::run as assertions;

use super::helpers::input;

#[test]
fn io_inbound_has_no_dependency_direction_rule() {
    let results = crate::run::check(&input());

    assertions::assert_no_finding_for_file(&results, "io/inbound/http/Cargo.toml");
}
