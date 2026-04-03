mod rule;
pub use rule::check;
pub(crate) use rule::has_owned_assertion_proof;

#[cfg(test)]

mod tests;
