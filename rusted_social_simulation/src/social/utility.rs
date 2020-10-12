use crate::social::condition::Condition;

/// A utility rule can be used to calculate the utility of something (e.g. an action) for a given context.
pub trait UtilityRule<T> {
    fn calculate_utility(&self, context: &T) -> i32;
}

/// An utility rule that has a fixed utility.
pub struct FixedUtility {
    utility: i32,
}

impl FixedUtility {
    pub fn new(utility: i32) -> FixedUtility {
        FixedUtility { utility }
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

/// An utility rule with an utility based on a condition.
pub struct ConditionalUtility<T> {
    condition: Box<dyn Condition<T>>,
    utility: i32,
}

impl<T> ConditionalUtility<T> {
    pub fn new(condition: Box<dyn Condition<T>>, utility: i32) -> ConditionalUtility<T> {
        ConditionalUtility { condition, utility }
    }
}

impl<T> UtilityRule<T> for ConditionalUtility<T> {
    /// Returns a fixed utility or 0 based on a condition.
    ///
    /// ```
    ///# use rusted_social_simulation::social::condition::*;
    ///# use rusted_social_simulation::social::utility::{ConditionalUtility, UtilityRule};
    /// let with_false = ConditionalUtility::new(Box::new(MockCondition::new(false)), 35);
    /// let with_true = ConditionalUtility::new(Box::new(MockCondition::new(true)), 78);
    ///
    /// assert_eq!(with_false.calculate_utility(&42), 0);
    /// assert_eq!(with_true.calculate_utility(&42), 78);
    /// ```
    fn calculate_utility(&self, context: &T) -> i32 {
        if self.condition.evaluate(context) {
            self.utility
        } else {
            0
        }
    }
}
