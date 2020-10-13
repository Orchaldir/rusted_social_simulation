use crate::social::condition::Condition;

type Utility = i32;

/// A utility rule can be used to calculate the utility of something (e.g. an action) for a given context.
pub trait UtilityRule<T> {
    fn calculate_utility(&self, context: &T) -> Utility;
}

/// An utility rule that has a fixed utility.
pub struct FixedUtility {
    utility: Utility,
}

impl FixedUtility {
    pub fn new(utility: Utility) -> FixedUtility {
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
    fn calculate_utility(&self, _: &T) -> Utility {
        self.utility
    }
}

/// An utility rule with an utility based on a condition.
pub struct ConditionalUtility<T> {
    condition: Box<dyn Condition<T>>,
    utility: Utility,
}

impl<T> ConditionalUtility<T> {
    pub fn new(condition: Box<dyn Condition<T>>, utility: Utility) -> ConditionalUtility<T> {
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
    fn calculate_utility(&self, context: &T) -> Utility {
        if self.condition.evaluate(context) {
            self.utility
        } else {
            0
        }
    }
}

/// The sum of multiple utility rules.
pub struct TotalUtility<T> {
    rules: Vec<Box<dyn UtilityRule<T>>>,
}

impl<T> TotalUtility<T> {
    pub fn new(rules: Vec<Box<dyn UtilityRule<T>>>) -> TotalUtility<T> {
        TotalUtility { rules }
    }
}

impl<T> UtilityRule<T> for TotalUtility<T> {
    /// Returns the sum of multiple utility rules.
    ///
    /// ```
    ///# use rusted_social_simulation::social::condition::*;
    ///# use rusted_social_simulation::social::utility::{UtilityRule, TotalUtility, FixedUtility};
    /// let sum = TotalUtility::new(vec![Box::new(FixedUtility::new(9)), Box::new(FixedUtility::new(5))]);
    ///
    /// assert_eq!(sum.calculate_utility(&42), 14);
    /// ```
    fn calculate_utility(&self, context: &T) -> Utility {
        self.rules
            .iter()
            .map(|r| r.calculate_utility(context))
            .sum()
    }
}

/// The maximum of multiple utility rules.
pub struct MaxUtility<T> {
    rules: Vec<Box<dyn UtilityRule<T>>>,
}

impl<T> MaxUtility<T> {
    pub fn new(rules: Vec<Box<dyn UtilityRule<T>>>) -> MaxUtility<T> {
        MaxUtility { rules }
    }
}

impl<T> UtilityRule<T> for MaxUtility<T> {
    /// Returns the maximum of multiple utility rules.
    ///
    /// ```
    ///# use rusted_social_simulation::social::condition::*;
    ///# use rusted_social_simulation::social::utility::{UtilityRule, MaxUtility, FixedUtility};
    /// let sum = MaxUtility::new(vec![Box::new(FixedUtility::new(9)), Box::new(FixedUtility::new(5))]);
    ///
    /// assert_eq!(sum.calculate_utility(&42), 9);
    /// ```
    fn calculate_utility(&self, context: &T) -> Utility {
        self.rules
            .iter()
            .map(|r| r.calculate_utility(context))
            .max()
            .unwrap_or(0)
    }
}
