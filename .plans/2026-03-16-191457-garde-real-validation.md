# Replace garde skip with real validation on CLI structs

**Date:** 2026-03-16 19:14
**Task:** Replace `#[garde(skip)]` with real validation where appropriate, and wire up `.validate()` calls.

## Goal
CLI args with meaningful validation constraints use garde custom validators instead of `skip`. Validation is actually called at runtime.

## Approach

### Step-by-step plan
1. **cli.rs**: Add `validate_format` custom validator function
2. **cli.rs**: Replace `#[garde(skip)]` on `format` field with `#[garde(custom(validate_format))]`
3. **cli.rs**: Update skip reason comments on fields that keep skip (boolean, path, etc.)
4. **main.rs**: Add `use garde::Validate;` and call `.validate(&())` on ValidateArgs in `handle_rs` and `handle_ts` where ValidateArgs are received
5. **commands/init.rs**: Add profile validation at top of `run_rs()`
6. Run cargo test

## Files to Modify
- `apps/guardrail3/src/cli.rs` — custom validator + annotation changes
- `apps/guardrail3/src/main.rs` — wire up validate() calls
- `apps/guardrail3/src/commands/init.rs` — profile validation at use site
