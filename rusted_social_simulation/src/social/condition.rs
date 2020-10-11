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
