#![expect(
    clippy::disallowed_methods,
    reason = "init_package_json is the centralized package.json writer for workspace init"
)]
#![allow(
    clippy::missing_docs_in_private_items,
    reason = "private init helpers are implementation details of package.json mutation"
)]

use std::path::Path;

use serde_json::{Map, Value};

use crate::fs as local_fs;

const PNPM_VERSION: &str = "10.32.0";
const SYNCPACK_VERSION: &str = "15.3.1";

type JsonObject = Map<String, Value>;

pub(crate) fn update_package_json_contract(
    root: &Path,
    force: bool,
    changes: &mut Vec<String>,
    refusals: &mut Vec<String>,
) {
    let rel = "package.json";
    let path = root.join(rel);
    let content = match local_fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            refusals.push(format!("refused to read {rel}\nreason: {error}"));
            return;
        }
    };
    let mut value = match serde_json::from_str::<Value>(&content) {
        Ok(Value::Object(map)) => map,
        Ok(_) => {
            refusals.push(format!(
                "refused to update {rel}\nreason: root JSON value is not an object"
            ));
            return;
        }
        Err(error) => {
            refusals.push(format!("refused to parse {rel}\nreason: {error}"));
            return;
        }
    };

    let mut package_changed = false;
    set_bool(
        &mut value,
        "private",
        true,
        force,
        &mut package_changed,
        refusals,
    );
    set_string(
        &mut value,
        "packageManager",
        format!("pnpm@{PNPM_VERSION}"),
        force,
        &mut package_changed,
        refusals,
    );
    update_package_engines(&mut value, force, &mut package_changed, refusals);
    update_package_scripts(&mut value, force, &mut package_changed, refusals);
    update_package_dev_dependencies(&mut value, force, &mut package_changed, refusals);
    update_package_pnpm(&mut value, force, &mut package_changed, refusals);

    if !refusals.is_empty() || !package_changed {
        return;
    }
    let output = match serde_json::to_string_pretty(&Value::Object(value)) {
        Ok(output) => format!("{output}\n"),
        Err(error) => {
            refusals.push(format!("refused to serialize {rel}\nreason: {error}"));
            return;
        }
    };
    if let Err(error) = local_fs::write(&path, &output) {
        refusals.push(format!("refused to write {rel}\nreason: {error}"));
    } else {
        changes.push(format!("updated {rel}"));
    }
}

fn update_package_engines(
    value: &mut JsonObject,
    force: bool,
    changed: &mut bool,
    refusals: &mut Vec<String>,
) {
    let Some(engines) = object_child(value, "engines", force, changed, refusals) else {
        return;
    };
    set_string(engines, "node", ">=24".to_owned(), force, changed, refusals);
    set_string(engines, "pnpm", "10".to_owned(), force, changed, refusals);
}

fn update_package_scripts(
    value: &mut JsonObject,
    force: bool,
    changed: &mut bool,
    refusals: &mut Vec<String>,
) {
    let Some(scripts) = object_child(value, "scripts", force, changed, refusals) else {
        return;
    };
    for (name, body) in [
        ("preinstall", "npx only-allow pnpm"),
        ("prepare", "g3ts validate repo --path ."),
        ("lint", "eslint --max-warnings 0 ."),
        ("typecheck", "tsc --noEmit"),
        ("lint:packages", "syncpack lint"),
        (
            "validate",
            "pnpm lint && pnpm typecheck && pnpm lint:packages",
        ),
    ] {
        set_string(scripts, name, body.to_owned(), force, changed, refusals);
    }
}

fn update_package_dev_dependencies(
    value: &mut JsonObject,
    force: bool,
    changed: &mut bool,
    refusals: &mut Vec<String>,
) {
    let Some(dev_dependencies) = object_child(value, "devDependencies", force, changed, refusals)
    else {
        return;
    };
    set_string(
        dev_dependencies,
        "syncpack",
        SYNCPACK_VERSION.to_owned(),
        force,
        changed,
        refusals,
    );
}

fn update_package_pnpm(
    value: &mut JsonObject,
    force: bool,
    changed: &mut bool,
    refusals: &mut Vec<String>,
) {
    let Some(pnpm) = object_child(value, "pnpm", force, changed, refusals) else {
        return;
    };
    if let Some(overrides) = object_child(pnpm, "overrides", force, changed, refusals) {
        set_string(
            overrides,
            "zod",
            "^4.0.0".to_owned(),
            force,
            changed,
            refusals,
        );
    }
    if !pnpm.contains_key("onlyBuiltDependencies") || force {
        let _ = pnpm.insert(
            "onlyBuiltDependencies".to_owned(),
            Value::Array(vec![Value::String("esbuild".to_owned())]),
        );
        *changed = true;
    }
}

fn object_child<'a>(
    map: &'a mut JsonObject,
    key: &str,
    force: bool,
    changed: &mut bool,
    refusals: &mut Vec<String>,
) -> Option<&'a mut JsonObject> {
    if !map.get(key).is_some_and(Value::is_object) {
        if map.contains_key(key) && !force {
            refusals.push(format!(
                "refused to update package.json\nreason: `{key}` exists but is not an object; rerun with --force"
            ));
            return None;
        }
        let _ = map.insert(key.to_owned(), Value::Object(Map::new()));
        *changed = true;
    }
    map.get_mut(key).and_then(Value::as_object_mut)
}

fn set_string(
    map: &mut JsonObject,
    key: &str,
    value: String,
    force: bool,
    changed: &mut bool,
    refusals: &mut Vec<String>,
) {
    set_value(map, key, Value::String(value), force, changed, refusals);
}

fn set_bool(
    map: &mut JsonObject,
    key: &str,
    value: bool,
    force: bool,
    changed: &mut bool,
    refusals: &mut Vec<String>,
) {
    set_value(map, key, Value::Bool(value), force, changed, refusals);
}

fn set_value(
    map: &mut JsonObject,
    key: &str,
    value: Value,
    force: bool,
    changed: &mut bool,
    refusals: &mut Vec<String>,
) {
    match map.get(key) {
        Some(existing) if existing == &value => {}
        Some(_) if force => {
            let _ = map.insert(key.to_owned(), value);
            *changed = true;
        }
        Some(_) => refusals.push(format!(
            "refused to update package.json\nreason: `{key}` already has a different value; rerun with --force"
        )),
        None => {
            let _ = map.insert(key.to_owned(), value);
            *changed = true;
        }
    }
}
