use g3rs_fmt_config_checks_assertions::extra_settings::rule as assertions;

use super::helpers::run_check;

#[test]
fn inventories_nonbaseline_settings() {
    let results = run_check(
        r#"
edition = "2024"
style_edition = "2024"
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
group_imports = "StdExternalCrate"
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "rustfmt extra setting: group_imports",
            "`group_imports` in `rustfmt.toml` is not part of the standard rustfmt baseline. Verify it is intentional.",
            "rustfmt.toml",
            true,
        )],
    );
}
