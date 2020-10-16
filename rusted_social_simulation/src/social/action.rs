use crate::social::condition::Condition;
use crate::social::effect::Effect;
use crate::social::utility::{Utility, UtilityRule};

/// An action that can be executed in a social simulation.
pub trait Action<T> {
    /// Can the action be executed with the current context?
    fn is_available(&self, context: &T) -> bool;

    /// What is the utility of the action with the current context?
    fn get_utility(&self, context: &T) -> Utility;

    /// Execute the action and change the current context.
    fn execute(&self, context: &mut T);
}

/// A simple implementation of Action.
pub struct SimpleAction<T> {
    condition: Box<dyn Condition<T>>,
    utility_rule: Box<dyn UtilityRule<T>>,
    effect: Box<dyn Effect<T>>,
}

impl<T> SimpleAction<T> {
    pub fn new(
        condition: Box<dyn Condition<T>>,
        utility_rule: Box<dyn UtilityRule<T>>,
        effect: Box<dyn Effect<T>>,
    ) -> SimpleAction<T> {
        SimpleAction {
            condition,
            utility_rule,
            effect,
        }
    }
}

impl<T> Action<T> for SimpleAction<T> {
    /// Evaluates a condition to check, if the action is available.
    ///
    /// ```
    ///# use rusted_social_simulation::social::condition::MockCondition;
    ///# use rusted_social_simulation::social::utility::FixedUtility;
    ///# use rusted_social_simulation::social::effect::DoNothing;
    ///# use rusted_social_simulation::social::action::{Action, SimpleAction};
    /// let condition = Box::new(MockCondition::new(true));
    /// let utility_rule = Box::new(FixedUtility::new(0));
    /// let effect = Box::new(DoNothing);
    /// let action = SimpleAction::new(condition, utility_rule, effect);
    ///
    /// assert!(action.is_available(&42));
    /// ```
    fn is_available(&self, context: &T) -> bool {
        self.condition.evaluate(context)
    }

    /// Uses an utility rule to calculate the action's utility.
    ///
    /// ```
    ///# use rusted_social_simulation::social::condition::MockCondition;
    ///# use rusted_social_simulation::social::utility::FixedUtility;
    ///# use rusted_social_simulation::social::effect::DoNothing;
    ///# use rusted_social_simulation::social::action::{Action, SimpleAction};
    /// let condition = Box::new(MockCondition::new(false));
    /// let utility_rule = Box::new(FixedUtility::new(13));
    /// let effect = Box::new(DoNothing);
    /// let action = SimpleAction::new(condition, utility_rule, effect);
    ///
    /// assert_eq!(action.get_utility(&42), 13);
    /// ```
    fn get_utility(&self, context: &T) -> Utility {
        self.utility_rule.calculate_utility(context)
    }

    /// Applies an effect onto the context to execute the action.
    ///
    /// ```
    ///# use rusted_social_simulation::social::condition::MockCondition;
    ///# use rusted_social_simulation::social::utility::FixedUtility;
    ///# use rusted_social_simulation::social::effect::{DoNothing, MockEffect};
    ///# use rusted_social_simulation::social::action::{Action, SimpleAction};
    /// let condition = Box::new(MockCondition::new(false));
    /// let utility_rule = Box::new(FixedUtility::new(0));
    /// let effect = Box::new(MockEffect::new(3));
    /// let action = SimpleAction::new(condition, utility_rule, effect);
    /// let mut context = 42;
    ///
    /// action.execute(&mut context);
    ///
    /// assert_eq!(context, 45);
    /// ```
    fn execute(&self, context: &mut T) {
        self.effect.apply(context)
    }
}
