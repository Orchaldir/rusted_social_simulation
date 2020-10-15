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
    fn apply(&self, context: &mut T) {
        for effect in  &self.effects {
            effect.apply(context)
        }
    }
}
