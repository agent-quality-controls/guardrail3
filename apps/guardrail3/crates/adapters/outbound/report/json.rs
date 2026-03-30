use serde_json::{Map, Number, Value, json};

use guardrail3_domain_report::{Report, Severity};

/// Strip the project root prefix from a file path to produce a relative path.
fn relative_path<'a>(file: &'a str, project_root: &str) -> &'a str {
    file.strip_prefix(project_root)
        .map_or(file, |s| s.strip_prefix('/').unwrap_or(s))
}

#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
pub fn print_report(report: &Report, _show_inventory: bool) {
    let project_root = report.project_path();

    let sections: Vec<Value> = report
        .sections()
        .iter()
        .map(|section| {
            // JSON always includes ALL results — consumers filter via the `inventory` field
            let results: Vec<Value> = section
                .results()
                .iter()
                .map(|r| {
                    let severity_str = match r.severity() {
                        Severity::Error => "error",
                        Severity::Warn => "warn",
                        Severity::Info => "info",
                    };

                    let mut obj = Map::new();
                    let _ = obj.insert("id".into(), Value::String(r.id().to_owned()));
                    let _ = obj.insert("severity".into(), Value::String(severity_str.into()));
                    let _ = obj.insert("title".into(), Value::String(r.title().to_owned()));
                    let _ = obj.insert("message".into(), Value::String(r.message().to_owned()));
                    let _ = obj.insert(
                        "file".into(),
                        match r.file() {
                            Some(f) => Value::String(f.to_owned()),
                            None => Value::Null,
                        },
                    );
                    let _ = obj.insert(
                        "file_relative".into(),
                        match r.file() {
                            Some(f) => Value::String(relative_path(f, project_root).to_owned()),
                            None => Value::Null,
                        },
                    );
                    let _ = obj.insert(
                        "line".into(),
                        match r.line() {
                            Some(l) => Value::Number(Number::from(l)),
                            None => Value::Null,
                        },
                    );
                    let _ = obj.insert("inventory".into(), Value::Bool(r.inventory()));
                    Value::Object(obj)
                })
                .collect();

            json!({
                "name": section.name(),
                "results": results,
            })
        })
        .collect();

    let output = json!({
        "project": report.project_path(),
        "stacks": report.stacks(),
        "sections": sections,
        "summary": {
            "errors": report.error_count(),
            "warnings": report.warn_count(),
            "info": report.info_count(),
        }
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&output).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
    );
}
