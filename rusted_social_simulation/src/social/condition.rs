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
    /// assert!(TrueCondition.evaluate(&42))
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
    /// assert!(!FalseCondition.evaluate(&42))
    /// ```
    fn evaluate(&self, _: &T) -> bool {
        false
    }
}

/// A condition that negates the evaluation of another condition
pub struct NotCondition<T> {
    condition: Box<dyn Condition<T>>,
}

impl<T> NotCondition<T> {
    pub fn new(condition: Box<dyn Condition<T>>) -> NotCondition<T> {
        NotCondition { condition }
    }
}

impl<T> Condition<T> for NotCondition<T> {
    /// Returns the negated evaluation of another conition.
    ///
    /// ```
    ///# use rusted_social_simulation::social::condition::*;
    /// let not_with_false = NotCondition::new(Box::new(FalseCondition));
    /// let not_with_true = NotCondition::new(Box::new(TrueCondition));
    ///
    /// assert!(not_with_false.evaluate(&42));
    /// assert!(!not_with_true.evaluate(&42))
    /// ```
    fn evaluate(&self, context: &T) -> bool {
        !self.condition.evaluate(context)
    }
}
