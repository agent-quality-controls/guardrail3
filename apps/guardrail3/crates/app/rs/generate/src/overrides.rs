use std::path::Path;

/// Local override fragments merged into generated Rust configs.
pub(crate) struct LocalOverrides {
    pub(crate) clippy_methods: String,
    pub(crate) clippy_types: String,
    pub(crate) deny_bans: String,
    pub(crate) deny_skip: String,
    pub(crate) deny_feature_bans: String,
}

pub(crate) fn load_local_overrides(project_path: &Path) -> LocalOverrides {
    let overrides_dir = project_path.join(".guardrail3/overrides");

    let read_and_validate = |name: &str| -> String {
        let path = overrides_dir.join(name);
        let raw = guardrail3_shared_fs::read_file(&path).unwrap_or_default();
        let clean = raw.strip_prefix('\u{FEFF}').unwrap_or(&raw);
        if clean.trim().is_empty() {
            return String::new();
        }
        validate_override_content(clean, name)
    };

    LocalOverrides {
        clippy_methods: read_and_validate("clippy-methods.toml"),
        clippy_types: read_and_validate("clippy-types.toml"),
        deny_bans: read_and_validate("deny-bans.toml"),
        deny_skip: read_and_validate("deny-skip.toml"),
        deny_feature_bans: read_and_validate("deny-feature-bans.toml"),
    }
}

/// Validate override content before injecting it into generated TOML.
#[allow(clippy::print_stderr)] // reason: malformed override warnings are user-facing CLI diagnostics
fn validate_override_content(content: &str, file_name: &str) -> String {
    let mut valid = String::new();
    let is_feature_bans = file_name.contains("feature-bans");
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let normalized = trimmed.replace(' ', "");
        let is_entry = normalized.starts_with("{path=") || normalized.starts_with("{name=");
        let is_section_header = is_feature_bans && normalized.starts_with("[[");
        if is_entry || is_section_header {
            valid.push_str(line);
            valid.push('\n');
        } else {
            eprintln!("  warning: skipping invalid line in {file_name}: {trimmed}");
        }
    }
    valid
}
