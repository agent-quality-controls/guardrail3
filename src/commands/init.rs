use std::fs;
use std::path::Path;

use crate::cli::InitArgs;

pub fn run(args: &InitArgs) {
    let project_path = Path::new(&args.path);

    let config_path = project_path.join("guardrail3.toml");
    if config_path.exists() && !args.force {
        eprintln!(
            "Error: guardrail3.toml already exists at {}",
            config_path.display()
        );
        eprintln!("Use --force to overwrite.");
        std::process::exit(1);
    }

    let config_content = format!(
        r#"version = "0.1"

[profile]
name = "{profile}"

[rust]
workspace_root = "."

[local]
clippy_methods = "local/clippy-methods.toml"
clippy_types = "local/clippy-types.toml"
deny_bans = "local/deny-bans.toml"
deny_skip = "local/deny-skip.toml"
"#,
        profile = args.profile
    );

    if let Err(e) = fs::write(&config_path, &config_content) {
        eprintln!("Error writing guardrail3.toml: {e}");
        std::process::exit(1);
    }

    // Create local/ directory with empty override files
    let local_dir = project_path.join("local");
    if let Err(e) = fs::create_dir_all(&local_dir) {
        eprintln!("Error creating local/ directory: {e}");
        std::process::exit(1);
    }

    let local_files = [
        (
            "clippy-methods.toml",
            "# Additional disallowed-methods entries (TOML array-of-tables format)\n# Example:\n#     { path = \"some::method\", reason = \"Use alternative instead\" },\n",
        ),
        (
            "clippy-types.toml",
            "# Additional disallowed-types entries (TOML array-of-tables format)\n# Example:\n#     { path = \"some::Type\", reason = \"Use alternative instead\" },\n",
        ),
        (
            "deny-bans.toml",
            "# Additional [bans] deny entries for deny.toml\n# Example:\n#     { name = \"some-crate\", wrappers = [] },\n",
        ),
        (
            "deny-skip.toml",
            "# Skip entries for deny.toml [bans] section\n# Example:\n#     { crate = \"windows-sys@0.60.2\", reason = \"transitive dep conflict\" },\n",
        ),
    ];

    for (filename, content) in &local_files {
        let file_path = local_dir.join(filename);
        if file_path.exists() && !args.force {
            println!("  Skipping existing: local/{filename}");
            continue;
        }
        if let Err(e) = fs::write(&file_path, content) {
            eprintln!("Error writing local/{filename}: {e}");
            std::process::exit(1);
        }
    }

    println!("Initialized guardrail3 project at {}", project_path.display());
    println!("  Created: guardrail3.toml (profile: {})", args.profile);
    println!("  Created: local/ directory with override files");
    println!();
    println!("Next steps:");
    println!("  1. Edit guardrail3.toml to configure your project");
    println!("  2. Add project-specific overrides in local/*.toml");
    println!("  3. Run: guardrail3 generate");
}
