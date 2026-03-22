use crate::domain::report::Severity;

use super::super::inputs::RustfmtRootInput;
use super::check;

#[test]
fn reports_nightly_only_keys_on_stable_toolchain() {
    let parsed = toml::from_str::<toml::Value>(
        r#"
edition = "2024"
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
group_imports = "StdExternalCrate"
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
        result.id == "RS-FMT-04"
            && result.severity == Severity::Warn
            && result.title == "nightly-only rustfmt setting `group_imports` on stable"
            && result.message
                == "`group_imports` is nightly-only, but rust-toolchain.toml uses `stable`."
            && result.file.as_deref() == Some("rustfmt.toml")
    }));
}
