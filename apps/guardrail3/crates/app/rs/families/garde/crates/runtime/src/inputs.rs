use super::facts::{GardeInputFailureFacts, GardeRootFacts};

pub struct GardeRootInput<'a> {
    pub(crate) root: &'a GardeRootFacts,
}

pub struct GardeInputFailureInput<'a> {
    pub(crate) failure: &'a GardeInputFailureFacts,
}

impl<'a> GardeRootInput<'a> {
    pub const fn new(root: &'a GardeRootFacts) -> Self {
        Self { root }
    }
}

impl<'a> GardeInputFailureInput<'a> {
    pub const fn new(failure: &'a GardeInputFailureFacts) -> Self {
        Self { failure }
    }
}
