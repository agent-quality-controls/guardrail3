use guardrail3_domain_report::{CheckResult, Severity};
use syn::spanned::Spanned;

const ID: &str = "RS-ARCH-09";

/// Scans all parsed source files for `#[path = "..."]` attributes on mod
/// declarations. Unconditionally forbidden — `#[path]` bypasses the module
/// facade (mod.rs) and breaks the module boundary model.
pub(crate) fn check_file(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
    rel_path: &str,
    results: &mut Vec<CheckResult>,
) {
    let content = if let Some(cached) = tree.file_content(rel_path) {
        cached.to_owned()
    } else {
        let abs = tree.abs_path(rel_path);
        match guardrail3_shared_fs::read_file_err(&abs) {
            Ok(c) => c,
            Err(_) => return,
        }
    };

    let Ok(ast) =
        syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(&content))
    else {
        return;
    };

    for item in &ast.items {
        let syn::Item::Mod(m) = item else {
            continue;
        };

        for attr in &m.attrs {
            if !attr.path().is_ident("path") {
                continue;
            }

            let path_value = extract_path_value(attr);
            let line = attr.span().start().line;

            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "#[path] attribute forbidden".to_owned(),
                format!(
                    "`#[path = \"{}\"]` on `mod {}` bypasses the module facade. \
                     Use standard module resolution with mod.rs instead. \
                     Every module directory must have a mod.rs that serves as its facade.",
                    path_value.as_deref().unwrap_or("..."),
                    m.ident
                ),
                Some(rel_path.to_owned()),
                Some(line),
                false,
            ));
        }
    }
}

fn extract_path_value(attr: &syn::Attribute) -> Option<String> {
    if let syn::Meta::NameValue(nv) = &attr.meta {
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(s),
            ..
        }) = &nv.value
        {
            return Some(s.value());
        }
    }
    None
}
