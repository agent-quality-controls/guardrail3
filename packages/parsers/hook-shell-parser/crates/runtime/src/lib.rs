#[cfg(feature = "api")]
pub mod command_query;
mod parser;
mod shell_ast;
mod support;
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use parser::parse_script;
