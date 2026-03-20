use std::fmt::Write;
use std::path::Path;

use crate::adapters::outbound::fs::RealFileSystem;
use crate::app::ts::validate::auto_detect_app_type;
use crate::app::ts::validate::ts_arch_checks::discover_ts_apps;
use crate::domain::report::TsAppType;
use crate::ports::outbound::FileSystem;

/// Initialize Rust guardrail3 configuration: creates guardrail3.toml with discovered workspace
/// members and per-crate config. Config files (clippy.toml, deny.toml, etc.) are created by `generate`.
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
        let config_path = project_path.join("guardrail3.toml");
        let config_content = generate_rs_config_content(profile, project_path);
        show_file_diff(&config_path, &config_content);
        println!(
            "\nNext: run `guardrail3 rs generate` to create config files (clippy.toml, deny.toml, etc.)"
        );
        return;
    }

    let mut created: Vec<String> = Vec::new();
    scaffold_config(project_path, profile, force, &mut created);
    println!("Initialized Rust guardrail3 at {}", project_path.display());
    for f in &created {
        println!("  Created: {f}");
    }
    println!(
        "\nNext: run `guardrail3 rs generate` to create config files (clippy.toml, deny.toml, etc.)"
    );
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
    if let Err(e) = crate::fs::write_file(
        &config_path,
        &generate_rs_config_content(profile, project_path),
    ) {
        eprintln!("Error writing guardrail3.toml: {e}");
        std::process::exit(1);
    }
    created.push(format!("guardrail3.toml (profile: {profile})"));
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
        if line.starts_with('[') && !line.starts_with("[typescript") {
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

#[allow(clippy::too_many_lines)] // reason: generates per-app config sections sequentially
fn generate_rs_config_content(profile: &str, project_path: &Path) -> String {
    let fs = RealFileSystem;
    let project = crate::app::core::discover::detect_project(&fs, project_path);

    let mut config = format!(
        r#"version = "0.1"

[profile]
name = "{profile}"

[rust]
workspace_root = "."
"#
    );

    let members = project.all_member_dirs();
    if members.is_empty() {
        // Single-crate — workspace-level checks
        writeln!(config, "\n[rust.checks]").unwrap_or_default();
        writeln!(config, "architecture = true      # R-ARCH-*, R-DEPS-*").unwrap_or_default();
        writeln!(config, "garde = true             # R-GARDE-*").unwrap_or_default();
        writeln!(config, "tests = true             # R-TEST-*").unwrap_or_default();
        writeln!(
            config,
            "release = true           # R-REL-*, R-PUB-*, R-BIN-*"
        )
        .unwrap_or_default();
    } else {
        // Multi-crate workspace — per-app + packages
        let mut seen_apps: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut has_packages = false;

        for ws in &project.workspaces {
            for member in &ws.members {
                let dir = &member.dir;
                if dir.starts_with("packages/") || dir.contains("/packages/") {
                    has_packages = true;
                    continue;
                }
                let app_name = if dir.starts_with("apps/") {
                    dir.strip_prefix("apps/")
                        .and_then(|rest| rest.split('/').next())
                        .unwrap_or(&member.name)
                } else {
                    &member.name
                };
                if !seen_apps.contains(app_name) {
                    let _ = seen_apps.insert(app_name.to_owned());
                    writeln!(config, "\n[rust.apps.{app_name}]").unwrap_or_default();
                    writeln!(config, "type = \"service\"").unwrap_or_default();
                    writeln!(config, "\n[rust.apps.{app_name}.checks]").unwrap_or_default();
                    writeln!(config, "architecture = true      # R-ARCH-*, R-DEPS-*")
                        .unwrap_or_default();
                    writeln!(config, "garde = true             # R-GARDE-*").unwrap_or_default();
                    writeln!(config, "tests = true             # R-TEST-*").unwrap_or_default();
                    writeln!(
                        config,
                        "release = true           # R-REL-*, R-PUB-*, R-BIN-*"
                    )
                    .unwrap_or_default();
                }
            }
        }
        if has_packages {
            writeln!(config, "\n[rust.packages]").unwrap_or_default();
            writeln!(config, "type = \"library\"").unwrap_or_default();
            writeln!(config, "\n[rust.packages.checks]").unwrap_or_default();
            writeln!(
                config,
                "architecture = false     # libraries don't use hex arch"
            )
            .unwrap_or_default();
            writeln!(
                config,
                "garde = false            # no input boundary validation for shared packages"
            )
            .unwrap_or_default();
            writeln!(config, "tests = true             # R-TEST-*").unwrap_or_default();
            writeln!(
                config,
                "release = false          # packages published with the app"
            )
            .unwrap_or_default();
        }
    }

    config
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
