use crate::modules;

pub fn list_modules() {
    let all = modules::all_modules();

    // Group by category prefix
    let mut current_category = String::new();

    for module in &all {
        let category = module
            .name
            .split('/')
            .next()
            .unwrap_or(module.name);

        if category != current_category {
            if !current_category.is_empty() {
                println!();
            }
            println!("=== {category} ===");
            current_category = category.to_string();
        }

        println!("  {:<40} {}", module.name, module.description);
    }
}

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
            eprintln!("Run 'guardrail3 list-modules' to see available modules.");
            std::process::exit(1);
        }
    }
}
