use std::collections::BTreeMap;

use colored::Colorize;

use crate::domain::report::{CheckResult, Report, Severity};

/// Maximum number of results per check ID before summarizing.
const VERBOSE_THRESHOLD: usize = 3;

#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
pub fn print_report(report: &Report, show_inventory: bool, verbose: bool) {
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

        if verbose {
            for result in &visible {
                print_result(result);
            }
        } else {
            print_with_summarization(&visible);
        }

        println!();
    }

    print_summary(report, show_inventory);
}

/// Print results, summarizing check IDs that exceed the threshold.
#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
fn print_with_summarization(results: &[&CheckResult]) {
    // Group by check ID preserving first-seen order
    let mut groups: BTreeMap<&str, Vec<&&CheckResult>> = BTreeMap::new();
    let mut order: Vec<&str> = Vec::new();
    for result in results {
        let id = result.id.as_str();
        let entry = groups.entry(id).or_default();
        if entry.is_empty() {
            order.push(id);
        }
        entry.push(result);
    }

    for id in &order {
        let group = &groups[*id];
        if group.len() <= VERBOSE_THRESHOLD {
            for result in group {
                print_result(result);
            }
        } else {
            print_summary_line(group);
        }
    }
}

/// Print a single summary line for a group of results with the same check ID.
#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
fn print_summary_line(group: &[&&CheckResult]) {
    let first = group[0];
    let count = group.len();

    let icon = match first.severity {
        Severity::Error => "\u{2717}".red().bold(),
        Severity::Warn => "\u{26a0}".yellow().bold(),
        Severity::Info => "\u{2139}".dimmed(),
    };

    let id_colored = match first.severity {
        Severity::Error => first.id.red().bold(),
        Severity::Warn => first.id.yellow(),
        Severity::Info => first.id.dimmed(),
    };

    println!(
        "  {} [{}] {}: {} entries (use --verbose to list each)",
        icon, id_colored, first.title, count,
    );
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
