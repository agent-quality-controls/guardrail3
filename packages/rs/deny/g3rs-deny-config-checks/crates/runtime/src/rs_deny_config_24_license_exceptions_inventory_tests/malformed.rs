use g3rs_deny_config_checks_assertions::rs_deny_config_24_license_exceptions_inventory as assertions;

use crate::test_support::run;

#[test]
fn inventories_documented_and_invalid_license_exceptions() {
    let results = run(
        r#"
[licenses]

[[licenses.exceptions]]
crate = "documented"
allow = ["MIT"]
reason = "Temporary exception while upstream relicensing lands."

[[licenses.exceptions]]
allow = ["MIT"]
reason = "Temporary exception while upstream relicensing lands."

[[licenses.exceptions]]
crate = "missing-reason"
allow = ["MIT"]

[[licenses.exceptions]]
crate = "blank-license"
allow = ["", "MIT"]
reason = "Temporary exception while upstream relicensing lands."

[[licenses.exceptions]]
crate = "weak-reason"
allow = ["MIT"]
reason = "temp"
"#,
        Some("service"),
        true,
        crate::rs_deny_config_24_license_exceptions_inventory::check,
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "license exception entry",
                "`deny.toml` has documented license exception for `documented`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "malformed license exception entry",
                "`deny.toml` has `[[licenses.exceptions]]` entry without a valid crate identifier.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "license exception missing reason",
                "`deny.toml` has license exception `missing-reason` without a `reason`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "malformed license exception entry",
                "`deny.toml` has `[[licenses.exceptions]]` entry `blank-license` with blank allowed license name.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "license exception reason too weak",
                "`deny.toml` has license exception `weak-reason` with a weak `reason`: reason must not be a placeholder.",
                "deny.toml",
                false,
            ),
            assertions::warn_no_file(
                "license exception count",
                "`deny.toml` has 3 license exceptions (1 documented, 1 missing or invalid reasons, 1 weak reasons).",
                false,
            ),
        ],
    );
}
