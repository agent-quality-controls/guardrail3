pub const MAX_STRUCT_BOOLS: i64 = 3;
pub const MAX_FN_PARAMS_BOOLS: i64 = 3;
pub const TOO_MANY_LINES_THRESHOLD: i64 = 75;
pub const TOO_MANY_ARGUMENTS_THRESHOLD: i64 = 7;
pub const EXCESSIVE_NESTING_THRESHOLD: i64 = 4;
pub const COGNITIVE_COMPLEXITY_THRESHOLD: i64 = 15;
pub const TYPE_COMPLEXITY_THRESHOLD: i64 = 75;

pub const THRESHOLD_VALUES: &[(&str, i64)] = &[
    ("max-struct-bools", MAX_STRUCT_BOOLS),
    ("max-fn-params-bools", MAX_FN_PARAMS_BOOLS),
    ("too-many-lines-threshold", TOO_MANY_LINES_THRESHOLD),
    ("too-many-arguments-threshold", TOO_MANY_ARGUMENTS_THRESHOLD),
    ("excessive-nesting-threshold", EXCESSIVE_NESTING_THRESHOLD),
    (
        "cognitive-complexity-threshold",
        COGNITIVE_COMPLEXITY_THRESHOLD,
    ),
    ("type-complexity-threshold", TYPE_COMPLEXITY_THRESHOLD),
];

pub const THRESHOLDS: &str = r"# Maximum lines per function before clippy::too_many_lines fires.
too-many-lines-threshold = 75

# Maximum cognitive complexity score per function.
cognitive-complexity-threshold = 15

# Maximum number of function parameters.
too-many-arguments-threshold = 7

# Maximum type nesting complexity score.
type-complexity-threshold = 75

# Maximum number of bool fields in a struct before clippy::struct_excessive_bools fires.
max-struct-bools = 3

# Maximum number of bool parameters in a function.
max-fn-params-bools = 3

# Maximum control-flow nesting depth before clippy::excessive_nesting fires.
excessive-nesting-threshold = 4";
