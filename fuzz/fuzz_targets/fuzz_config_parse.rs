#![no_main]
//! Fuzz target for guardrail3 TOML config parsing.
//!
//! Feeds arbitrary byte sequences as TOML input to the config deserializer.
//! Goal: deserialization must never panic, only return Ok or Err.

use libfuzzer_sys::fuzz_target;

use guardrail3::config::types::GuardrailConfig;

fuzz_target!(|data: &[u8]| {
    // Only try valid UTF-8 — TOML is text-based
    if let Ok(s) = std::str::from_utf8(data) {
        // Try parsing as guardrail3 config — must not panic
        let _ = toml::from_str::<GuardrailConfig>(s);

        // Also try parsing as generic TOML Value and then extracting fields
        // the way discover.rs does it
        let _ = s.parse::<toml::Value>();
    }
});
