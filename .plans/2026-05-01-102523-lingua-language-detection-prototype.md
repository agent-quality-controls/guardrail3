# Lingua language detection prototype

## Goal

Build a throwaway Rust prototype to evaluate whether `lingua` can detect language mismatches in static content pages before designing any G3TS rule.

## Approach

- Keep the prototype under `.plans/prototypes/lingua-language-detection`.
- Do not wire it into `apps`, `packages`, G3TS, hooks, or CI.
- Restrict Lingua to the seven target languages: English, Russian, German, French, Spanish, Portuguese, Indonesian.
- Run detection on:
  - short UI strings
  - longer article bodies
  - mixed page samples with route/UI language separate from body language
  - ambiguous short strings
- Print dominant detection, confidence values, and multi-language spans where available.

## Questions

- Is Lingua reliable enough for short UI controls?
- Is Lingua reliable on long article bodies?
- Can Lingua expose a useful mismatch signal when slug/UI language differs from article body?
- Should a future guardrail look at whole-page text, extracted body text, or separated page regions?

## Files

- `.plans/prototypes/lingua-language-detection/Cargo.toml`
- `.plans/prototypes/lingua-language-detection/src/main.rs`
- `.plans/prototypes/lingua-language-detection/README.md`

## Verification

- `cargo run --manifest-path .plans/prototypes/lingua-language-detection/Cargo.toml`
