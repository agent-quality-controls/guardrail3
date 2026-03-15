use super::types::{Report, Severity};

pub fn print_report(report: &Report) {
    println!("# Guardrail3 Validation Report");
    println!();
    println!("**Project:** {}", report.project_path);
    println!("**Stacks:** {}", report.stacks.join(", "));

    for section in &report.sections {
        println!();
        println!("## {}", section.name);
        println!();

        if section.results.is_empty() {
            println!("No checks in this section.");
            continue;
        }

        println!("| Status | ID | Title | Message | Location |");
        println!("|--------|-----|-------|---------|----------|");

        for result in &section.results {
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
    println!("| Info | {} |", report.info_count());
}
