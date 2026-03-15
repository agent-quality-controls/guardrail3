use colored::Colorize;

use super::types::{CheckResult, Report, Severity};

pub fn print_report(report: &Report) {
    println!();
    println!(
        "{}",
        format!(
            "Guardrail Report: {}",
            report.project_path
        )
        .bold()
    );
    println!(
        "Stacks: {}",
        report.stacks.join(", ").cyan()
    );
    println!();

    for section in &report.sections {
        println!(
            "{} {} {}",
            "===".bold(),
            section.name.bold(),
            "===".bold()
        );
        if section.results.is_empty() {
            println!("  {} No checks in this section", "(empty)".dimmed());
        }
        for result in &section.results {
            print_result(result);
        }
        println!();
    }

    print_summary(report);
}

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
        (Some(f), Some(l)) => format!(" ({}:{})", f, l),
        (Some(f), None) => format!(" ({})", f),
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

fn print_summary(report: &Report) {
    let errors = report.error_count();
    let warns = report.warn_count();
    let infos = report.info_count();

    println!("{}", "=== Summary ===".bold());
    println!(
        "  {} errors, {} warnings, {} info",
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
        infos.to_string().dimmed()
    );
    println!();
}
