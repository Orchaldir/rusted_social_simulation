use crate::social::action::Action;
use crate::social::practice::role::Role;
use std::collections::HashMap;

pub mod role;

/// A template for a social practice.
pub trait PracticeTemplate<T> {
    /// Gets all actions of a role in this practice.
    fn get_actions(&self, role: Role) -> Vec<&dyn Action<T>>;

    /// Gets the name of this practice.
    fn get_name(&self) -> &str;

    /// Gets all roles that participate in this practice.
    fn get_roles(&self) -> &Vec<Role>;

    /// Gets the name of a role, if it participates in this practice.
    fn get_role_name(&self, role: Role) -> Option<&str>;
}

/// A social practice.
pub trait Practice<T> {
    /// Gets all actions of an entity in this practice.
    fn get_actions(&self, entity: u32) -> Vec<&dyn Action<T>>;

    /// Gets all roles that participate in this practice.
    fn get_roles(&self) -> &HashMap<Role, u32>;

    /// Gets the template of this practice.
    fn get_template(&self) -> &dyn PracticeTemplate<T>;
}
