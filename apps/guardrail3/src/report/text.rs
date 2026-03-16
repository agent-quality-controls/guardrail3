use colored::Colorize;

use crate::domain::report::{CheckResult, Report, Severity};

#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
pub fn print_report(report: &Report, show_inventory: bool) {
    println!();
    println!(
        "{}",
        format!("Guardrail Report: {}", report.project_path).bold()
    );
    println!("Stacks: {}", report.stacks.join(", ").cyan());
    println!();

    for section in &report.sections {
        let visible: Vec<&CheckResult> = section
            .results
            .iter()
            .filter(|r| show_inventory || !r.inventory)
            .collect();
        if visible.is_empty() && section.results.is_empty() {
            println!("{} {} {}", "===".bold(), section.name.bold(), "===".bold());
            println!("  {} No checks in this section", "(empty)".dimmed());
            println!();
            continue;
        }
        if visible.is_empty() {
            // All results were inventory — skip section entirely
            continue;
        }
        println!("{} {} {}", "===".bold(), section.name.bold(), "===".bold());
        for result in &visible {
            print_result(result);
        }
        println!();
    }

    print_summary(report, show_inventory);
}

#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
fn print_result(result: &CheckResult) {
    let icon = match result.severity {
        Severity::Error => "\u{2717}".red().bold(),
        Severity::Warn => "\u{26a0}".yellow().bold(),
        Severity::Info => "\u{2139}".dimmed(),
    };

    let id_colored = match result.severity {
        Severity::Error => result.id.red().bold(),
        Severity::Warn => result.id.yellow(),
        Severity::Info => result.id.dimmed(),
    };

    let location = match (&result.file, result.line) {
        (Some(f), Some(l)) => format!(" ({f}:{l})"),
        (Some(f), None) => format!(" ({f})"),
        _ => String::new(),
    };

    println!(
        "  {} [{}] {}: {}{}",
        icon,
        id_colored,
        result.title,
        result.message,
        location.dimmed()
    );
}

#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
fn print_summary(report: &Report, show_inventory: bool) {
    let errors = report.error_count();
    let warns = report.warn_count();
    let infos = report.info_count();
    let inventory_hidden = report.inventory_count();

    println!("{}", "=== Summary ===".bold());

    let info_display = if !show_inventory && inventory_hidden > 0 {
        format!(
            "{} info ({} hidden, use --inventory to show all)",
            infos, inventory_hidden
        )
    } else {
        format!("{infos} info")
    };

    println!(
        "  {} errors, {} warnings, {}",
        if errors > 0 {
            errors.to_string().red().bold()
        } else {
            errors.to_string().green().bold()
        },
        if warns > 0 {
            warns.to_string().yellow().bold()
        } else {
            warns.to_string().green().bold()
        },
        info_display.dimmed()
    );
    println!();
}
