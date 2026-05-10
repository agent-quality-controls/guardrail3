# Lingua Language Detection Prototype

Throwaway prototype for checking whether `lingua` is useful for Astro/i18n page-language mismatch detection.

This is not a G3TS package and is not wired into any app.

Run:

```sh
cargo run --manifest-path .plans/prototypes/lingua-language-detection/Cargo.toml
```

The prototype restricts detection to:

- English
- Russian
- German
- French
- Spanish
- Portuguese
- Indonesian

It prints:

- dominant language per sample
- confidence values for all seven languages
- multi-language spans for mixed samples
- a simple mismatch signal comparing declared route/UI language against body language

Current read:

- long body/article text is reliable for all seven target languages in the sample set
- aggregated short UI text can be reliable, but single-token UI text is ambiguous
- page-level dominant detection catches Spanish route/UI with English body
- region-level body detection is the cleaner mismatch signal
- raw multi-language span detection is noisy for Spanish/Portuguese and should not be used as the main signal
