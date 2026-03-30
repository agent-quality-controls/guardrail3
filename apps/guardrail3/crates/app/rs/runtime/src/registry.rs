use guardrail3_validation_model::RustValidateFamily;

use crate::runners::{compiled_runners, RustFamilyRunnerDef};

pub(crate) fn runner_for(family: RustValidateFamily) -> Option<RustFamilyRunnerDef> {
    compiled_runners()
        .into_iter()
        .find(|runner| runner.family == family)
}
