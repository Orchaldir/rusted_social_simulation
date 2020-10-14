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
