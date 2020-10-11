/// A condition that can evaluate to true or false given a context.
pub trait Condition<T> {
    fn evaluate(&self, context: &T) -> bool;
}

/// A condition that always evaluates to true.
pub struct TrueCondition;

impl<T> Condition<T> for TrueCondition {
    /// Always returns true.
    ///
    /// ```
    ///# use rusted_social_simulation::social::condition::{TrueCondition, Condition};
    /// let condition = TrueCondition{};
    /// assert!(condition.evaluate(&42))
    /// ```
    fn evaluate(&self, _: &T) -> bool {
        true
    }
}

/// A condition that always evaluates to false.
pub struct FalseCondition;

impl<T> Condition<T> for FalseCondition {
    /// Always returns false.
    ///
    /// ```
    ///# use rusted_social_simulation::social::condition::{FalseCondition, Condition};
    /// let condition = FalseCondition{};
    /// assert!(!condition.evaluate(&42))
    /// ```
    fn evaluate(&self, _: &T) -> bool {
        false
    }
}
