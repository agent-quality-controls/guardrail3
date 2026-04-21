use g3ts_arch_types::G3TsArchConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ARCH-CONFIG-03";

pub(crate) fn check(input: &G3TsArchConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = crate::support::parsed_manifest(input) else {
        return;
    };

    for entrypoint in &snapshot.declared_entrypoints {
        if crate::support::canonical_entrypoint(&entrypoint.rel_path) {
            results.push(crate::support::info(
                ID,
                "declared facade entrypoint is canonical",
                format!(
                    "Declared facade entrypoint `{}` uses the canonical package facade location.",
                    entrypoint.rel_path
                ),
                &snapshot.rel_path,
            ));
            continue;
        }

        results.push(crate::support::error(
            ID,
            "declared facade entrypoint is not canonical",
            format!(
                "Declared facade entrypoint `{}` is not canonical. Use `src/index.ts`, `src/index.tsx`, `index.ts`, or `index.tsx`.",
                entrypoint.rel_path
            ),
            &snapshot.rel_path,
        ));
    }
}
