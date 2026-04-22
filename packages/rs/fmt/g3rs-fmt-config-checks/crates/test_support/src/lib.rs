mod input;

#[cfg(feature = "support")]
pub use g3rs_fmt_types::{
    G3RsFmtCargoFacts, G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtRustPolicyState,
    G3RsFmtRustfmtConfigState, G3RsFmtRustfmtFacts, G3RsFmtToolchainFacts, G3RsFmtToolchainState,
    G3RsFmtWaiver,
};

#[cfg(feature = "support")]
pub use input::{
    explicit_keys, parsed_cargo, parsed_rustfmt, parsed_toolchain, rustfmt_input, waiver,
};
