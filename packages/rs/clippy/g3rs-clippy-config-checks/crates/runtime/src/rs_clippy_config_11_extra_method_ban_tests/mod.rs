use crate::rs_clippy_config_11_extra_method_ban::check;
use crate::test_support::{findings, input_from_raw};

#[test]
fn inventories_extra_method_ban() {
    let input = input_from_raw(
        "clippy.toml",
        "disallowed-methods = [{ path = \"example::extra\", reason = \"project-specific\" }]\n",
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "extra method ban" && finding.message.contains("example::extra")
    }));
}

#[test]
fn inventories_clean_state_when_no_extra_method_bans_exist() {
    let input = input_from_raw("clippy.toml", "disallowed-methods = []\n");
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "no extra method bans" && finding.inventory
    }));
}
