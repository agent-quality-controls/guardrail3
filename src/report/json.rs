use serde_json::{Map, Number, Value, json};

use super::types::{Report, Severity};

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
                    obj.insert("id".into(), Value::String(r.id.clone()));
                    obj.insert(
                        "severity".into(),
                        Value::String(severity_str.into()),
                    );
                    obj.insert("title".into(), Value::String(r.title.clone()));
                    obj.insert(
                        "message".into(),
                        Value::String(r.message.clone()),
                    );
                    obj.insert(
                        "file".into(),
                        match &r.file {
                            Some(f) => Value::String(f.clone()),
                            None => Value::Null,
                        },
                    );
                    obj.insert(
                        "line".into(),
                        match r.line {
                            Some(l) => {
                                Value::Number(Number::from(l))
                            }
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
        serde_json::to_string_pretty(&output)
            .unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
    );
}
