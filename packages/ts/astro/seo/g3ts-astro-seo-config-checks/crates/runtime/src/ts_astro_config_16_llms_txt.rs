use g3ts_astro_seo_types::G3TsAstroSeoIntegrationContractInput;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-SEO-CONFIG-16";

pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    if let Some(rel_path) = &contract.llms_txt_rel_path {
        results.push(crate::support::info(
            ID,
            "LLMs discovery file exists",
            format!("Found `{rel_path}`."),
            rel_path,
        ));
        return;
    }

    let expected_path = if contract.app_root_rel_path == "." {
        "public/llms.txt".to_owned()
    } else {
        format!("{}/public/llms.txt", contract.app_root_rel_path)
    };
    results.push(crate::support::error(
            ID,
            "Astro public content app is missing `public/llms.txt`",
            format!(
                "Astro public content app `{}` must contain committed file `{expected_path}`. Route-generated `llms.txt` does not satisfy the default contract because G3TS must catch the missing discovery file before build.",
                contract.app_root_rel_path
            ),
            Some(&expected_path),
        ));
}
