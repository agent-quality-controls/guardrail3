use guardrail3_app_commands::command_ids::RS_LIST_MODULES;
use guardrail3_domain_modules as modules;

pub fn list_modules() -> String {
    let all = modules::all_modules();

    // Group by category prefix
    let mut current_category = String::new();
    let mut output = String::new();

    for module in &all {
        let category = module.name.split('/').next().unwrap_or(module.name);

        if category != current_category {
            if !current_category.is_empty() {
                output.push('\n');
            }
            output.push_str(&format!("=== {category} ===\n"));
            category.clone_into(&mut current_category);
        }

        output.push_str(&format!("  {:<40} {}\n", module.name, module.description));
    }

    output
}

pub fn show_module(name: &str) -> Result<String, String> {
    match modules::find_module(name) {
        Some(module) => Ok(format!(
            "# Module: {}\n# {}\n\n{}",
            module.name, module.description, module.content
        )),
        None => Err(format!(
            "Error: module '{name}' not found.\nRun '{RS_LIST_MODULES}' to see available modules."
        )),
    }
}
