use std::collections::BTreeMap;

use colored::Colorize;

use guardrail3_domain_report::{CheckResult, Report, Severity};

/// Maximum number of results per check ID before summarizing.
const VERBOSE_THRESHOLD: usize = 3;

/// Strip the project root prefix from a file path to produce a relative path.
fn relative_path<'a>(file: &'a str, project_root: &str) -> &'a str {
    file.strip_prefix(project_root)
        .map_or(file, |s| s.strip_prefix('/').unwrap_or(s))
}

#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
pub fn print_report(report: &Report, show_inventory: bool, verbose: bool) {
    println!();
    println!(
        "{}",
        format!("Guardrail Report: {}", report.project_path()).bold()
    );
    println!("Stacks: {}", report.stacks().join(", ").cyan());
    println!();

    let project_root = report.project_path();

    for section in report.sections() {
        let visible: Vec<&CheckResult> = section
            .results()
            .iter()
            .filter(|r| show_inventory || !r.inventory())
            .collect();
        if visible.is_empty() {
            // No visible results — skip section entirely (whether empty or all inventory)
            continue;
        }
        println!(
            "{} {} {}",
            "===".bold(),
            section.name().bold(),
            "===".bold()
        );

        if verbose {
            for result in &visible {
                print_result(result, project_root);
            }
        } else {
            print_with_summarization(&visible, project_root);
        }

        println!();
    }

    print_summary(report, show_inventory);
}

/// Print results, summarizing check IDs that exceed the threshold.
#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
fn print_with_summarization(results: &[&CheckResult], project_root: &str) {
    // Group by check ID preserving first-seen order
    let mut groups: BTreeMap<&str, Vec<&CheckResult>> = BTreeMap::new();
    let mut order: Vec<&str> = Vec::new();
    for result in results.iter().copied() {
        let id = result.id();
        let entry = groups.entry(id).or_default();
        if entry.is_empty() {
            order.push(id);
        }
        entry.push(result);
    }

    for id in &order {
        if let Some(group) = groups.get(*id) {
            if group.len() <= VERBOSE_THRESHOLD {
                for result in group {
                    print_result(result, project_root);
                }
            } else {
                print_summary_line(group);
            }
        }
    }
}

/// Print a single summary line for a group of results with the same check ID.
#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
fn print_summary_line(group: &[&CheckResult]) {
    let Some(first) = group.first() else { return };
    let count = group.len();

    let icon = match first.severity() {
        Severity::Error => "\u{2717}".red().bold(),
        Severity::Warn => "\u{26a0}".yellow().bold(),
        Severity::Info => "\u{2139}".dimmed(),
    };

    let id_colored = match first.severity() {
        Severity::Error => first.id().red().bold(),
        Severity::Warn => first.id().yellow(),
        Severity::Info => first.id().dimmed(),
    };

    println!(
        "  {} [{}] {}: {count} (use --verbose to list each)",
        icon,
        id_colored,
        first.title(),
    );
}

#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
fn print_result(result: &CheckResult, project_root: &str) {
    let icon = match result.severity() {
        Severity::Error => "\u{2717}".red().bold(),
        Severity::Warn => "\u{26a0}".yellow().bold(),
        Severity::Info => "\u{2139}".dimmed(),
    };

    let id_colored = match result.severity() {
        Severity::Error => result.id().red().bold(),
        Severity::Warn => result.id().yellow(),
        Severity::Info => result.id().dimmed(),
    };

    let location = match (result.file(), result.line()) {
        (Some(f), Some(l)) => format!(" ({}:{l})", relative_path(f, project_root)),
        (Some(f), None) => format!(" ({})", relative_path(f, project_root)),
        _ => String::new(),
    };

    println!(
        "  {} [{}] {}: {}{}",
        icon,
        id_colored,
        result.title(),
        result.message(),
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
        format!("{infos} info ({inventory_hidden} hidden, use --inventory to show all)")
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
