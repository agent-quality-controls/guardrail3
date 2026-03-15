use serde_json::{Map, Number, Value, json};

use super::types::{Report, Severity};

#[allow(clippy::print_stdout)] // reason: CLI report output to stdout
pub fn print_report(report: &Report) {
    let sections: Vec<Value> = report
        .sections
        .iter()
        .map(|section| {
            let results: Vec<Value> = section
                .results
                .iter()
                .map(|r| {
                    let severity_str = match r.severity {
                        Severity::Error => "error",
                        Severity::Warn => "warn",
                        Severity::Info => "info",
                    };

                    let mut obj = Map::new();
                    let _ = obj.insert("id".into(), Value::String(r.id.clone()));
                    let _ = obj.insert("severity".into(), Value::String(severity_str.into()));
                    let _ = obj.insert("title".into(), Value::String(r.title.clone()));
                    let _ = obj.insert("message".into(), Value::String(r.message.clone()));
                    let _ = obj.insert(
                        "file".into(),
                        match &r.file {
                            Some(f) => Value::String(f.clone()),
                            None => Value::Null,
                        },
                    );
                    let _ = obj.insert(
                        "line".into(),
                        match r.line {
                            Some(l) => Value::Number(Number::from(l)),
                            None => Value::Null,
                        },
                    );
                    Value::Object(obj)
                })
                .collect();

            json!({
                "name": section.name,
                "results": results,
            })
        })
        .collect();

    let output = json!({
        "project": report.project_path,
        "stacks": report.stacks,
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
