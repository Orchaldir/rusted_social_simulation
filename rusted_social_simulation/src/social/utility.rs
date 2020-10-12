/// A utility rule can be used to calculate the utility of something (e.g. an action) for a given context.
pub trait UtilityRule<T> {
    fn calculate_utility(&self, context: &T) -> i32;
}

/// An utility rule that has a fixed utility.
pub struct FixedUtility {
    utility: i32,
}

impl FixedUtility {
    pub fn new(value: i32) -> FixedUtility {
        FixedUtility { utility: value }
    }
}

impl<T> UtilityRule<T> for FixedUtility {
    /// Always returns a fixed utility.
    ///
    /// ```
    ///# use rusted_social_simulation::social::utility::{FixedUtility, UtilityRule};
    /// assert_eq!(FixedUtility::new(9).calculate_utility(&42), 9);
    /// ```
    fn calculate_utility(&self, _: &T) -> i32 {
        self.utility
    }
}
