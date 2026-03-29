crate::define_rule_assertions!("RS-DEPS-11");

pub fn assert_summary<T>(actual: T, expected: T)
where
    T: core::fmt::Debug + PartialEq,
{
    assert_eq!(actual, expected);
}
