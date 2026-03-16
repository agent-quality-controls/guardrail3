use crate::domain::report::{Report, Severity};

#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
pub fn print_report(report: &Report, show_inventory: bool) {
    println!("# Guardrail3 Validation Report");
    println!();
    println!("**Project:** {}", report.project_path);
    println!("**Stacks:** {}", report.stacks.join(", "));

    for section in &report.sections {
        let visible: Vec<_> = section
            .results
            .iter()
            .filter(|r| show_inventory || !r.inventory)
            .collect();

        if visible.is_empty() && section.results.is_empty() {
            println!();
            println!("## {}", section.name);
            println!();
            println!("No checks in this section.");
            continue;
        }
        if visible.is_empty() {
            continue;
        }

        println!();
        println!("## {}", section.name);
        println!();

        println!("| Status | ID | Title | Message | Location |");
        println!("|--------|-----|-------|---------|----------|");

        for result in &visible {
            let icon = match result.severity {
                Severity::Error => "\u{2717}",
                Severity::Warn => "\u{26a0}",
                Severity::Info => "\u{2139}",
            };

            let location = match (&result.file, result.line) {
                (Some(f), Some(l)) => format!("{f}:{l}"),
                (Some(f), None) => f.clone(),
                _ => String::new(),
            };

            // Escape pipe characters in user-provided strings
            let title = result.title.replace('|', "\\|");
            let message = result.message.replace('|', "\\|");
            let location = location.replace('|', "\\|");

            println!(
                "| {icon} | {} | {title} | {message} | {location} |",
                result.id,
            );
        }
    }

    println!();
    println!("## Summary");
    println!();
    println!("| Level | Count |");
    println!("|-------|-------|");
    println!("| Errors | {} |", report.error_count());
    println!("| Warnings | {} |", report.warn_count());

    let inventory_hidden = report.inventory_count();
    if !show_inventory && inventory_hidden > 0 {
        println!(
            "| Info | {} ({} hidden, use --inventory to show all) |",
            report.info_count(),
            inventory_hidden
        );
    } else {
        println!("| Info | {} |", report.info_count());
    }
}
