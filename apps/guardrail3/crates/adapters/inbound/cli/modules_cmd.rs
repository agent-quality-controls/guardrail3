use crate::domain::modules;
use guardrail3_app_commands::command_ids::RS_LIST_MODULES;

#[allow(clippy::print_stdout)] // reason: CLI command — module listing output to stdout
pub fn list_modules() {
    let all = modules::all_modules();

    // Group by category prefix
    let mut current_category = String::new();

    for module in &all {
        let category = module.name.split('/').next().unwrap_or(module.name);

        if category != current_category {
            if !current_category.is_empty() {
                println!();
            }
            println!("=== {category} ===");
            category.clone_into(&mut current_category);
        }

        println!("  {:<40} {}", module.name, module.description);
    }
}

#[allow(clippy::print_stdout, clippy::print_stderr, clippy::disallowed_methods)] // reason: CLI command — user-facing output and exit codes
pub fn show_module(name: &str) {
    match modules::find_module(name) {
        Some(module) => {
            println!("# Module: {}", module.name);
            println!("# {}", module.description);
            println!();
            println!("{}", module.content);
        }
        None => {
            eprintln!("Error: module '{name}' not found.");
            eprintln!("Run '{RS_LIST_MODULES}' to see available modules.");
            std::process::exit(1);
        }
    }
}
