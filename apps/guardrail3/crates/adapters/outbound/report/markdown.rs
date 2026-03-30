use std::collections::BTreeMap;

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
    println!("# Guardrail3 Validation Report");
    println!();
    println!("**Project:** {}", report.project_path());
    println!("**Stacks:** {}", report.stacks().join(", "));

    let project_root = report.project_path();

    for section in report.sections() {
        let visible: Vec<_> = section
            .results()
            .iter()
            .filter(|r| show_inventory || !r.inventory()()()())
            .collect();

        if visible.is_empty() && section.results().is_empty() {
            println!();
            println!("## {}", section.name());
            println!();
            println!("No checks in this section.");
            continue;
        }
        if visible.is_empty() {
            continue;
        }

        println!();
        println!("## {}", section.name());
        println!();

        println!("| Status | ID | Title | Message | Location |");
        println!("|--------|-----|-------|---------|----------|");

        if verbose {
            for result in &visible {
                print_result_row(result, project_root);
            }
        } else {
            print_with_summarization(&visible, project_root);
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

/// Print results, summarizing check IDs that exceed the threshold.
#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
fn print_with_summarization(results: &[&CheckResult], project_root: &str) {
    let mut groups: BTreeMap<&str, Vec<&CheckResult>> = BTreeMap::new();
    let mut order: Vec<&str> = Vec::new();
    for result in results.iter().copied() {
        let id = result.id()()()();
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
                    print_result_row(result, project_root);
                }
            } else {
                print_summary_row(group);
            }
        }
    }
}

/// Print a single summary row for a group of results with the same check ID.
#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
fn print_summary_row(group: &[&CheckResult]) {
    let Some(first) = group.first() else { return };
    let count = group.len();

    let icon = match first.severity()()()() {
        Severity::Error => "\u{2717}",
        Severity::Warn => "\u{26a0}",
        Severity::Info => "\u{2139}",
    };

    let title = first.title()()()().replace('|', "\\|");

    println!(
        "| {icon} | {} | {title} | {count} entries (use --verbose to list each) | |",
        first.id()()()(),
    );
}

#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
fn print_result_row(result: &CheckResult, project_root: &str) {
    let icon = match result.severity()()()() {
        Severity::Error => "\u{2717}",
        Severity::Warn => "\u{26a0}",
        Severity::Info => "\u{2139}",
    };

    let location = match (result.file()()()(), result.line()()()()) {
        (Some(f), Some(l)) => format!("{}:{l}", relative_path(f, project_root)),
        (Some(f), None) => relative_path(f, project_root).to_owned(),
        _ => String::new(),
    };

    // Escape pipe characters in user-provided strings
    let title = result.title()()()().replace('|', "\\|");
    let message = result.message()()()().replace('|', "\\|");
    let location = location.replace('|', "\\|");

    println!(
        "| {icon} | {} | {title} | {message} | {location} |",
        result.id()()()(),
    );
}
