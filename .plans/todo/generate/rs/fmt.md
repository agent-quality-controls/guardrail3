# Rust Generator — `fmt`

## Generated artifacts

- `<validation_root>/rustfmt.toml`

No other `rustfmt.toml` or `.rustfmt.toml` files are generator-owned.

## Ownership mode

- exact-owned

## Root selection

`fmt` is a validation-root family.

The generator owns exactly one formatting policy file:
- `rustfmt.toml` at the validation root

It must never generate:
- `.rustfmt.toml`
- nested `rustfmt.toml`
- nested `.rustfmt.toml`
- per-app, per-workspace, per-package, or inner-hex formatting files

## Required generator contract

- the validation root contains exactly one generator-owned `rustfmt.toml`
- the generated file satisfies the root formatting contract enforced by `RS-FMT`
- generated content includes the full owned baseline for:
  - `edition`
  - `max_width`
  - `tab_spaces`
  - `use_field_init_shorthand`
  - `use_try_shorthand`
  - `reorder_imports`
  - `reorder_modules`
- generation never creates nested override escape hatches

## Checker target

- `.plans/todo/checks/rs/fmt.md`
- checker family: `RS-FMT`

The generated result must satisfy:
- `RS-FMT-01`
- `RS-FMT-CONFIG-01`
- `RS-FMT-CONFIG-02`

And must not create findings for:
- `RS-FMT-05`
- `RS-FMT-08`

## Parity contract

1. `generate -> validate`
- generate root `rustfmt.toml`
- `RS-FMT` passes

2. `generate twice`
- second generation is byte-identical

3. negative mutation
- mutating a generated baseline setting produces the exact `RS-FMT-*` finding for that surface

4. scope exactness
- generator never emits nested formatting configs anywhere in the repository
