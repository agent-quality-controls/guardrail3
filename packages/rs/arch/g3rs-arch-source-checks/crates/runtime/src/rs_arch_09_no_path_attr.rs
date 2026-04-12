use g3rs_arch_types::G3RsArchSourceFile;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use syn::spanned::Spanned;

const ID: &str = "RS-ARCH-SOURCE-09";

pub(crate) fn check_file(file: &G3RsArchSourceFile, results: &mut Vec<G3CheckResult>) {
    let Ok(ast) = syn::parse_file(file.content.strip_prefix('\u{feff}').unwrap_or(&file.content))
    else {
        return;
    };

    for item in &ast.items {
        let syn::Item::Mod(module) = item else {
            continue;
        };

        for attr in &module.attrs {
            if !attr.path().is_ident("path") {
                continue;
            }

            let path_value = extract_path_value(attr);
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "#[path] attribute forbidden".to_owned(),
                format!(
                    "`#[path = \"{}\"]` on `mod {}` bypasses the module facade. Use standard module resolution with mod.rs instead. Every module directory must have a mod.rs that serves as its facade.",
                    path_value.as_deref().unwrap_or("..."),
                    module.ident
                ),
                Some(file.rel_path.clone()),
                Some(attr.span().start().line),
            ));
        }
    }
}

fn extract_path_value(attr: &syn::Attribute) -> Option<String> {
    if let syn::Meta::NameValue(name_value) = &attr.meta {
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(value),
            ..
        }) = &name_value.value
        {
            return Some(value.value());
        }
    }
    None
}
