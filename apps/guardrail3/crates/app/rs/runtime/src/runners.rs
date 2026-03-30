use guardrail3_domain_report::CheckResult;
use guardrail3_validation_model::RustValidateFamily;

use crate::context::RustRunContext;

pub(crate) type RunnerFn = for<'a> fn(&RustRunContext<'a>) -> Vec<CheckResult>;

pub(crate) struct RustFamilyRunnerDef {
    pub(crate) family: RustValidateFamily,
    pub(crate) run: RunnerFn,
}

#[cfg(feature = "family-arch")]
fn run_arch(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_arch::check(ctx.tree, &ctx.mapper.map_rs_arch())
}

#[cfg(feature = "family-fmt")]
fn run_fmt(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_fmt::check(ctx.tree)
}

#[cfg(feature = "family-toolchain")]
fn run_toolchain(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_toolchain::check(ctx.tree, &ctx.mapper.map_rs_toolchain())
}

#[cfg(feature = "family-clippy")]
fn run_clippy(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_clippy::check(ctx.tree, &ctx.mapper.map_rs_clippy())
}

#[cfg(feature = "family-deny")]
fn run_deny(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_deny::check(ctx.tree, &ctx.mapper.map_rs_deny())
}

#[cfg(feature = "family-cargo")]
fn run_cargo(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_cargo::check(ctx.tree, &ctx.mapper.map_rs_cargo())
}

#[cfg(feature = "family-code")]
fn run_code(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_code::check(ctx.tree, &ctx.mapper.map_rs_code())
}

#[cfg(feature = "family-hexarch")]
fn run_hexarch(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_hexarch::check(ctx.tree, &ctx.mapper.map_rs_hexarch())
}

#[cfg(feature = "family-libarch")]
fn run_libarch(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_libarch::check(ctx.tree, &ctx.mapper.map_rs_libarch())
}

#[cfg(feature = "family-deps")]
fn run_deps(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_deps::check(ctx.tree, &ctx.mapper.map_rs_deps(), ctx.tc)
}

#[cfg(feature = "family-garde")]
fn run_garde(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_garde::check(ctx.tree, &ctx.mapper.map_rs_garde())
}

#[cfg(feature = "family-test")]
fn run_test(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_test::check(ctx.tree, &ctx.mapper.map_rs_test(), ctx.tc)
}

#[cfg(feature = "family-release")]
fn run_release(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_release::check(
        ctx.tree,
        &ctx.mapper.map_rs_release(),
        ctx.tc,
        ctx.thorough,
    )
}

#[cfg(feature = "family-hooks-shared")]
fn run_hooks_shared(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_hooks_shared::check(ctx.fs, ctx.path, ctx.tree, ctx.tc)
}

#[cfg(feature = "family-hooks-rs")]
fn run_hooks_rs(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    guardrail3_app_rs_family_hooks_rs::check(ctx.tree, ctx.tc)
}

pub(crate) fn compiled_runners() -> Vec<RustFamilyRunnerDef> {
    let mut runners = Vec::new();

    #[cfg(feature = "family-arch")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Arch,
        run: run_arch,
    });

    #[cfg(feature = "family-fmt")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Fmt,
        run: run_fmt,
    });

    #[cfg(feature = "family-toolchain")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Toolchain,
        run: run_toolchain,
    });

    #[cfg(feature = "family-clippy")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Clippy,
        run: run_clippy,
    });

    #[cfg(feature = "family-deny")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Deny,
        run: run_deny,
    });

    #[cfg(feature = "family-cargo")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Cargo,
        run: run_cargo,
    });

    #[cfg(feature = "family-code")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Code,
        run: run_code,
    });

    #[cfg(feature = "family-hexarch")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Hexarch,
        run: run_hexarch,
    });

    #[cfg(feature = "family-libarch")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Libarch,
        run: run_libarch,
    });

    #[cfg(feature = "family-deps")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Deps,
        run: run_deps,
    });

    #[cfg(feature = "family-garde")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Garde,
        run: run_garde,
    });

    #[cfg(feature = "family-test")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Test,
        run: run_test,
    });

    #[cfg(feature = "family-release")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Release,
        run: run_release,
    });

    #[cfg(feature = "family-hooks-shared")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::HooksShared,
        run: run_hooks_shared,
    });

    #[cfg(feature = "family-hooks-rs")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::HooksRs,
        run: run_hooks_rs,
    });

    runners
}
