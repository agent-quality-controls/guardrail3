use g3ts_arch_types::{G3TsArchFacadeFileState, G3TsArchSourceChecksInput};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3ts-arch/facade-only";

pub(crate) fn check(input: &G3TsArchSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    for facade in &input.facades {
        let G3TsArchFacadeFileState::Parsed { surface } = facade else {
            continue;
        };

        for item in &surface.body_items {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "facade file must be facade-only".to_owned(),
                format!(
                    "Facade file `{}` contains {} `{}`. Move implementation into a sibling module and re-export it from the facade.",
                    surface.rel_path, item.kind, item.name
                ),
                Some(surface.rel_path.clone()),
                Some(item.line),
            ));
        }

        if surface.body_items.is_empty() {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "facade file is facade-only".to_owned(),
                    format!(
                        "Facade file `{}` contains only facade declarations.",
                        surface.rel_path
                    ),
                    Some(surface.rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
        }
    }
}
