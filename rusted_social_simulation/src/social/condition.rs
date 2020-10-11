/// A condition that can evaluate to true or false given a context.
pub trait Condition<T> {
    fn evaluate(&self, context: &T) -> bool;
}

/// A condition that always evaluates to a fixed value.
pub struct MockCondition {
    value: bool,
}

impl MockCondition {
    pub fn new(value: bool) -> MockCondition {
        MockCondition { value }
    }
}

impl<T> Condition<T> for MockCondition {
    /// Always returns a fixed value.
    ///
    /// ```
    ///# use rusted_social_simulation::social::condition::{MockCondition, Condition};
    /// assert!(MockCondition::new(true).evaluate(&42));
    /// assert!(!MockCondition::new(false).evaluate(&42));
    /// ```
    fn evaluate(&self, _: &T) -> bool {
        self.value
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
    /// let not_with_false = NotCondition::new(Box::new(MockCondition::new(false)));
    /// let not_with_true = NotCondition::new(Box::new(MockCondition::new(true)));
    ///
    /// assert!(not_with_false.evaluate(&42));
    /// assert!(!not_with_true.evaluate(&42))
    /// ```
    fn evaluate(&self, context: &T) -> bool {
        !self.condition.evaluate(context)
    }
}

/// A condition that evaluates to true, if all sub-conditions are true.
pub struct AndCondition<T> {
    conditions: Vec<Box<dyn Condition<T>>>,
}

impl<T> AndCondition<T> {
    pub fn new(conditions: Vec<Box<dyn Condition<T>>>) -> AndCondition<T> {
        AndCondition { conditions }
    }
}

impl<T> Condition<T> for AndCondition<T> {
    /// Returns true, if all sub-conditions are true.
    ///
    /// ```
    ///# use rusted_social_simulation::social::condition::*;
    /// let true0 = Box::new(MockCondition::new(true));
    /// let true1 = Box::new(MockCondition::new(true));
    ///
    /// assert!(AndCondition::new(vec![true0, true1]).evaluate(&42));
    /// ```
    fn evaluate(&self, context: &T) -> bool {
        for condition in &self.conditions {
            if !condition.evaluate(context) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_and(false, false, false, false);
        assert_and(true, false, false, false);
        assert_and(false, true, false, false);
        assert_and(true, true, false, false);
        assert_and(false, false, true, false);
        assert_and(true, false, true, false);
        assert_and(false, true, true, false);
        assert_and(true, true, true, true);
    }

    fn assert_and(value0: bool, value1: bool, value2: bool, result: bool) {
        let c0 = Box::new(MockCondition::new(value0));
        let c1 = Box::new(MockCondition::new(value1));
        let c2 = Box::new(MockCondition::new(value2));

        assert_eq!(AndCondition::new(vec![c0, c1, c2]).evaluate(&42), result);
    }
}
