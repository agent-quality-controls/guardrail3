use super::check;
use crate::test_support::{findings, input_from_raw};

#[test]
fn inventories_extra_type_ban() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-types = [{ path = \"example::extra\", reason = \"project-specific\" }]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "extra type ban" && finding.message.contains("example::extra")
    }));
}

#[test]
fn inventories_clean_state_when_no_extra_type_bans_exist() {
    let input = input_from_raw("clippy.toml", "disallowed-types = []\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "no extra type bans" && finding.inventory
    }));
}

#[test]
fn reports_malformed_type_sections() {
    let input = input_from_raw("clippy.toml", "disallowed-types = [1]\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "disallowed-types section malformed"
            && finding.message.contains("disallowed-types[0]")
    }));
}
