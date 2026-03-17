use std::fmt::Write;
use std::path::Path;

use crate::adapters::outbound::fs::RealFileSystem;
use crate::app::ts::validate::auto_detect_app_type;
use crate::app::ts::validate::ts_arch_checks::discover_ts_apps;
use crate::domain::report::TsAppType;
use crate::ports::outbound::FileSystem;

/// Initialize Rust guardrail3 configuration: creates guardrail3.toml with [rust] + [local] sections,
/// creates local/ directory with Rust override files, and scaffolds release config for service profile.
#[allow(clippy::print_stdout)] // reason: CLI command — user-facing output
#[allow(clippy::print_stderr)] // reason: CLI command — error output
#[allow(clippy::disallowed_methods)] // reason: CLI command — exit codes and fs operations
pub fn run_rs(profile: &str, path: &str, force: bool, dry_run: bool) {
    if profile != "service" && profile != "library" {
        eprintln!("Error: unknown profile '{profile}'. Must be 'service' or 'library'.");
        std::process::exit(1);
    }
    let project_path = Path::new(path);

    if dry_run {
        println!("Dry run — rs init --profile {profile}\n");

        // Show the guardrail3.toml config (the important part)
        let config_path = project_path.join("guardrail3.toml");
        let config_content = generate_rs_config_content(profile);
        show_file_diff(&config_path, &config_content);

        // Just list other files that would be created (no content dump)
        let other_files = [
            "local/clippy-methods.toml",
            "local/clippy-types.toml",
            "local/deny-bans.toml",
            "local/deny-skip.toml",
            "local/deny-feature-bans.toml",
        ];
        println!("\n  Would also create:");
        for f in &other_files {
            let full = project_path.join(f);
            if full.exists() {
                println!("    {f} (already exists, skip without --force)");
            } else {
                println!("    {f} (override template)");
            }
        }
        if profile == "service" {
            for f in &["release-plz.toml", "cliff.toml"] {
                let full = project_path.join(f);
                if full.exists() {
                    println!("    {f} (already exists, skip without --force)");
                } else {
                    println!("    {f} (release config)");
                }
            }
        }
        return;
    }

    let mut created: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();

    scaffold_config(project_path, profile, force, &mut created);
    scaffold_local_dir(project_path, force, &mut created, &mut skipped);
    if profile == "service" {
        scaffold_release_files(project_path, force, &mut created, &mut skipped);
    }
    print_rs_summary(project_path, &created, &skipped);
}

/// Write guardrail3.toml config file.
#[allow(clippy::print_stderr)] // reason: CLI command — error output
#[allow(clippy::disallowed_methods)] // reason: CLI command — exit codes
fn scaffold_config(project_path: &Path, profile: &str, force: bool, created: &mut Vec<String>) {
    let config_path = project_path.join("guardrail3.toml");
    if config_path.exists() && !force {
        eprintln!(
            "Error: guardrail3.toml already exists at {}",
            config_path.display()
        );
        eprintln!("Use --force to overwrite.");
        std::process::exit(1);
    }
    if let Err(e) = crate::fs::write_file(&config_path, &generate_rs_config_content(profile)) {
        eprintln!("Error writing guardrail3.toml: {e}");
        std::process::exit(1);
    }
    created.push(format!("guardrail3.toml (profile: {profile})"));
}

/// Create local/ directory with override template files.
#[allow(clippy::print_stderr)] // reason: CLI command — error output
#[allow(clippy::disallowed_methods)] // reason: CLI command — exit codes
fn scaffold_local_dir(
    project_path: &Path,
    force: bool,
    created: &mut Vec<String>,
    skipped: &mut Vec<String>,
) {
    let local_dir = project_path.join("local");
    if let Err(e) = crate::fs::create_dir_all(&local_dir) {
        eprintln!("Error creating local/ directory: {e}");
        std::process::exit(1);
    }

    let local_files = [
        "clippy-methods.toml",
        "clippy-types.toml",
        "deny-bans.toml",
        "deny-skip.toml",
        "deny-feature-bans.toml",
    ];

    let local_templates = [
        "# Additional disallowed-methods entries (TOML array-of-tables format)\n# Example:\n#     { path = \"some::method\", reason = \"Use alternative instead\" },\n",
        "# Additional disallowed-types entries (TOML array-of-tables format)\n# Example:\n#     { path = \"some::Type\", reason = \"Use alternative instead\" },\n",
        "# Additional [bans] deny entries for deny.toml\n# Example:\n#     { name = \"some-crate\", wrappers = [] },\n",
        "# Skip entries for deny.toml [bans] section\n# Example:\n#     { crate = \"windows-sys@0.60.2\", reason = \"transitive dep conflict\" },\n",
        "# Additional [[bans.features]] entries for deny.toml\n# Example:\n#     [[bans.features]]\n#     name = \"some-crate\"\n#     deny = [\"full\"]\n",
    ];

    for (filename, content) in local_files.iter().zip(local_templates.iter()) {
        let file_path = local_dir.join(filename);
        if file_path.exists() && !force {
            skipped.push(format!("local/{filename}"));
            continue;
        }
        if let Err(e) = crate::fs::write_file(&file_path, content) {
            eprintln!("Error writing local/{filename}: {e}");
            std::process::exit(1);
        }
        created.push(format!("local/{filename}"));
    }
}

/// Write release config files (release-plz.toml, cliff.toml) for service profile.
#[allow(clippy::print_stderr)] // reason: CLI command — error output
#[allow(clippy::disallowed_methods)] // reason: CLI command — exit codes
fn scaffold_release_files(
    project_path: &Path,
    force: bool,
    created: &mut Vec<String>,
    skipped: &mut Vec<String>,
) {
    let release_files = [
        (
            "release-plz.toml",
            crate::domain::modules::release::RELEASE_PLZ_TOML.content,
        ),
        (
            "cliff.toml",
            crate::domain::modules::release::CLIFF_TOML.content,
        ),
    ];

    for (filename, content) in &release_files {
        let file_path = project_path.join(filename);
        if file_path.exists() && !force {
            skipped.push((*filename).to_owned());
            continue;
        }
        if let Err(e) = crate::fs::write_file(&file_path, content) {
            eprintln!("Error writing {filename}: {e}");
            std::process::exit(1);
        }
        created.push((*filename).to_owned());
    }
}

/// Print the summary of what was created/skipped.
#[allow(clippy::print_stdout)] // reason: CLI command — user-facing output
fn print_rs_summary(project_path: &Path, created: &[String], skipped: &[String]) {
    println!(
        "Initialized Rust guardrail3 project at {}",
        project_path.display()
    );
    for f in created {
        println!("  Created: {f}");
    }
    for f in skipped {
        println!("  Skipped (already exists): {f}");
    }
    if !skipped.is_empty() {
        println!("  Use --force to overwrite existing files.");
    }
    println!();
    println!("Next steps:");
    println!("  1. Edit guardrail3.toml to set workspace_root and crate layers");
    println!("  2. Add project-specific overrides in local/*.toml");
    println!("  3. Run: guardrail3 rs generate");
}

/// Initialize TypeScript guardrail3 configuration: appends [typescript] section to existing
/// guardrail3.toml, or creates a minimal one with only [typescript] if none exists.
#[allow(clippy::print_stdout)] // reason: CLI command — user-facing output
#[allow(clippy::print_stderr)] // reason: CLI command — error output
#[allow(clippy::disallowed_methods)] // reason: CLI command — exit codes and fs operations
pub fn run_ts(path: &str, force: bool, dry_run: bool) {
    let project_path = Path::new(path);
    let config_path = project_path.join("guardrail3.toml");

    // Analyze project to discover apps and their types
    let fs = RealFileSystem;
    let ts_section = generate_ts_section(&fs, project_path);

    if dry_run {
        if config_path.exists() {
            let existing = crate::fs::read_file(&config_path).unwrap_or_default();
            let new_content = if existing.contains("[typescript]") {
                replace_typescript_section(&existing, &ts_section)
            } else {
                format!("{existing}{ts_section}")
            };
            println!("Dry run — showing what ts init would do:\n");
            show_file_diff(&config_path, &new_content);
        } else {
            let config_content = format!("version = \"0.1\"\n{ts_section}");
            println!("Dry run — showing what ts init would do:\n");
            show_file_diff(&config_path, &config_content);
        }
        return;
    }

    if config_path.exists() {
        // Read existing content and check if [typescript] section already exists
        let existing = crate::fs::read_file(&config_path).unwrap_or_default();
        if existing.contains("[typescript]") && !force {
            eprintln!("Error: [typescript] section already exists in guardrail3.toml");
            eprintln!("Use --force to overwrite.");
            std::process::exit(1);
        }

        if existing.contains("[typescript]") {
            // Force mode: replace existing [typescript] section
            // Find [typescript] and replace everything from there to next section or EOF
            let new_content = replace_typescript_section(&existing, &ts_section);
            if let Err(e) = crate::fs::write_file(&config_path, &new_content) {
                eprintln!("Error writing guardrail3.toml: {e}");
                std::process::exit(1);
            }
        } else {
            // Append [typescript] section
            let new_content = format!("{existing}{ts_section}");
            if let Err(e) = crate::fs::write_file(&config_path, &new_content) {
                eprintln!("Error writing guardrail3.toml: {e}");
                std::process::exit(1);
            }
        }

        println!("Added [typescript] section to guardrail3.toml");
    } else {
        // Create minimal guardrail3.toml with only [typescript]
        let config_content = format!("version = \"0.1\"\n{ts_section}");
        if let Err(e) = crate::fs::write_file(&config_path, &config_content) {
            eprintln!("Error writing guardrail3.toml: {e}");
            std::process::exit(1);
        }
        println!(
            "Initialized TypeScript guardrail3 project at {}",
            project_path.display()
        );
        println!("  Created: guardrail3.toml (typescript only)");
    }

    println!();
    println!("Next steps:");
    println!("  1. Edit guardrail3.toml to configure your TypeScript apps");
    println!("  2. Run: guardrail3 ts generate");
}

/// Replace an existing [typescript] section in the config content.
/// Collects lines before `[typescript]`, skips the old section, inserts the new one,
/// then appends any lines from the next section onward.
fn replace_typescript_section(existing: &str, new_ts_section: &str) -> String {
    let mut result = String::new();
    let mut lines = existing.lines().peekable();
    let mut found_ts = false;

    // Copy lines before [typescript]
    while let Some(line) = lines.peek() {
        if line.trim() == "[typescript]" {
            found_ts = true;
            break;
        }
        result.push_str(lines.next().unwrap_or_default());
        result.push('\n');
    }

    if !found_ts {
        return format!("{existing}{new_ts_section}");
    }

    // Skip old [typescript] section (header + body lines until next [section] or EOF)
    let _ = lines.next(); // skip [typescript] line
    while let Some(line) = lines.peek() {
        if line.starts_with('[') {
            break;
        }
        let _ = lines.next();
    }

    // Insert new [typescript] section
    result.push_str(new_ts_section.trim_start_matches('\n'));
    result.push('\n');

    // Copy remaining sections
    for line in lines {
        result.push_str(line);
        result.push('\n');
    }

    result
}

/// Show a simple diff of what would change for a single file during dry run.
#[allow(clippy::print_stdout)] // reason: CLI dry-run output
fn show_file_diff(path: &Path, new_content: &str) {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    if path.exists() {
        let existing = crate::fs::read_file(path).unwrap_or_default();
        if existing == new_content {
            println!("  No changes to {name}");
        } else {
            println!("  Would update {name}:");
            for line in existing.lines() {
                if !new_content.contains(line) {
                    println!("    - {line}");
                }
            }
            for line in new_content.lines() {
                if !existing.contains(line) {
                    println!("    + {line}");
                }
            }
        }
    } else {
        println!("  Would create {name}:");
        for line in new_content.lines() {
            println!("    {line}");
        }
    }
}

fn generate_rs_config_content(profile: &str) -> String {
    match profile {
        "library" => format!(
            r#"version = "0.1"

[profile]
name = "{profile}"

# Library profile: all crates are treated as "pure" — global-state bans apply everywhere.
# No [crates.*] section needed unless you want per-crate method/type overrides.

[rust]
workspace_root = "."

[rust.checks]
architecture = true      # R-ARCH-*, R-DEPS-* — hex arch structure and dependency flow
garde = true             # R-GARDE-*, R34/R35 — input boundary validation (requires garde crate)
tests = true             # R-TEST-* — test quality and organization
release = true           # R-REL-*, R-PUB-*, R-BIN-* — crate publish readiness

[local]
clippy_methods = "local/clippy-methods.toml"
clippy_types = "local/clippy-types.toml"
deny_bans = "local/deny-bans.toml"
deny_skip = "local/deny-skip.toml"
deny_feature_bans = "local/deny-feature-bans.toml"
"#
        ),
        _ => format!(
            r#"version = "0.1"

[profile]
name = "{profile}"

[rust]
workspace_root = "."

[rust.checks]
architecture = true      # R-ARCH-*, R-DEPS-* — hex arch structure and dependency flow
garde = true             # R-GARDE-*, R34/R35 — input boundary validation (requires garde crate)
tests = true             # R-TEST-* — test quality and organization
release = true           # R-REL-*, R-PUB-*, R-BIN-* — crate publish readiness

[local]
clippy_methods = "local/clippy-methods.toml"
clippy_types = "local/clippy-types.toml"
deny_bans = "local/deny-bans.toml"
deny_skip = "local/deny-skip.toml"
deny_feature_bans = "local/deny-feature-bans.toml"
"#
        ),
    }
}

/// Generate the `[typescript]` TOML section by discovering apps and auto-detecting their types.
/// Each app gets explicit check flags based on its detected type. No global `[typescript.checks]`.
fn generate_ts_section(fs: &dyn FileSystem, project_path: &Path) -> String {
    let apps = discover_ts_apps(fs, project_path);

    let mut section = String::from("\n[typescript]\n");

    if apps.is_empty() {
        section.push_str("\n[typescript.apps.my-app]\n");
        section.push_str("type = \"service\"         # service | content | library\n");
        section.push_str("\n[typescript.apps.my-app.checks]\n");
        section.push_str("architecture = true      # T-ARCH-* — hex arch enforcement\n");
        section
            .push_str("content = false          # T-STYL-*, T-ESLP-07/08 — accessibility, SEO\n");
        section.push_str("tests = true             # T-TEST-* — test quality\n");
    } else {
        for app_path in &apps {
            let name = app_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            let detected_type = auto_detect_app_type(fs, app_path);
            let type_name = match detected_type {
                Some(TsAppType::Content) => "content",
                Some(TsAppType::Library) => "library",
                _ => "service",
            };
            let cats = match detected_type {
                Some(TsAppType::Content) => ("false", "true", "true"),
                Some(TsAppType::Library) => ("false", "false", "true"),
                _ => ("true", "false", "true"),
            };

            let reason = detect_reason(app_path, detected_type);

            writeln!(section, "\n[typescript.apps.{name}]").unwrap_or_default();
            writeln!(section, "type = \"{type_name}\"         # {reason}").unwrap_or_default();
            writeln!(section, "\n[typescript.apps.{name}.checks]").unwrap_or_default();
            writeln!(
                section,
                "architecture = {:<9}# T-ARCH-* — hex arch enforcement",
                cats.0
            )
            .unwrap_or_default();
            writeln!(
                section,
                "content = {:<14}# T-STYL-*, T-ESLP-07/08 — accessibility, SEO",
                cats.1
            )
            .unwrap_or_default();
            writeln!(section, "tests = {:<16}# T-TEST-* — test quality", cats.2)
                .unwrap_or_default();
        }
    }

    section
}

/// Return a human-readable reason for the detected app type, used as a TOML comment.
fn detect_reason(app_path: &Path, detected: Option<TsAppType>) -> &'static str {
    match detected {
        Some(TsAppType::Content) => {
            if app_path.join("content").is_dir() {
                "auto-detected: content/ directory"
            } else {
                "auto-detected: content dependencies"
            }
        }
        Some(TsAppType::Service) => {
            if app_path.join("src/modules/domain").is_dir() {
                "auto-detected: hex arch structure"
            } else {
                "auto-detected: backend framework"
            }
        }
        Some(TsAppType::Library) => "auto-detected: library",
        None => "default (no auto-detection signal)",
    }
}
