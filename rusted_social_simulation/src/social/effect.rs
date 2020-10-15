/// An effect that can modify the context.
pub trait Effect<T> {
    fn apply(&self, context: &mut T);
}

/// An effect that does nothing.
pub struct DoNothing;

impl<T> Effect<T> for DoNothing {
    /// Do nothing.
    ///
    /// ```
    ///# use rusted_social_simulation::social::effect::{DoNothing, Effect};
    /// let mut context = 42;
    /// DoNothing{}.apply(&mut context);
    /// assert_eq!(context, 42);
    /// ```
    fn apply(&self, _: &mut T) {}
}

/// A mock effect for testing.
pub struct MockEffect {
    value: u32,
}

impl MockEffect {
    pub fn new(value: u32) -> MockEffect {
        MockEffect { value }
    }
}

impl Effect<u32> for MockEffect {
    /// Modify the context for easy validating that the effect was called.
    ///
    /// ```
    ///# use rusted_social_simulation::social::effect::{Effect, MockEffect};
    /// let mut context = 42;
    /// let effect =  MockEffect::new(11);
    /// effect.apply(&mut context);
    /// assert_eq!(context, 53);
    /// ```
    fn apply(&self, context: &mut u32) {
        *context += self.value
    }
}

/// An effect that consists of multiple sub-effects.
pub struct EffectVector<T> {
    effects: Vec<Box<dyn Effect<T>>>,
}

impl<T> EffectVector<T> {
    pub fn new(effects: Vec<Box<dyn Effect<T>>>) -> EffectVector<T> {
        EffectVector { effects }
    }
}

impl<T> Effect<T> for EffectVector<T> {
    /// Apply multiple sub-effects.
    ///
    /// ```
    ///# use rusted_social_simulation::social::effect::{MockEffect, Effect, EffectVector};
    /// let mut context = 42;
    /// let effect0 = MockEffect::new(2);
    /// let effect1 = MockEffect::new(34);
    /// let vector = EffectVector::new(vec![Box::new(effect0), Box::new(effect1)]);
    ///
    /// vector.apply(&mut context);
    ///
    /// assert_eq!(context, 78);
    /// ```
    fn apply(&self, context: &mut T) {
        for effect in &self.effects {
            effect.apply(context)
        }
    }
}
