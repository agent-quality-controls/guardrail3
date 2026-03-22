use crate::domain::report::Severity;

use super::super::inputs::RustfmtRootInput;
use super::check;

#[test]
fn inventories_extra_nonstandard_root_settings() {
    let parsed = toml::from_str::<toml::Value>(
        r#"
edition = "2024"
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
newline_style = "Unix"
"#,
    )
    .expect("valid TOML");
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml"),
        parsed: Some(&parsed),
        workspace_edition: Some("2024"),
        toolchain_channel: Some("stable"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-FMT-03"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "rustfmt extra setting: newline_style"
            && result.message == "Non-baseline rustfmt setting present"
            && result.file.as_deref() == Some("rustfmt.toml")
    }));
}
