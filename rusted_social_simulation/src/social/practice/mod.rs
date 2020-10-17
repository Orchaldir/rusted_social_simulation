use crate::social::action::Action;
use crate::social::practice::role::Role;
use std::collections::HashMap;

pub mod role;

/// A template for a social practice.
pub trait PracticeTemplate<T> {
    /// Gets all actions of a role in this practice template.
    fn get_actions(&self, role: Role) -> Vec<&dyn Action<T>>;

    /// Gets the id of this practice template.
    fn get_id(&self) -> u32;

    /// Gets the name of this practice template.
    fn get_name(&self) -> &str;

    /// Gets all roles that participate in this practice template.
    fn get_roles(&self) -> &Vec<Role>;

    /// Gets the name of a role in this practice template.
    fn get_role_name(&self, role: Role) -> &str;
}

/// A social practice, which is an instance of a template.
pub trait Practice<T> {
    /// Gets all actions of an entity in this practice.
    fn get_actions(&self, entity: u32) -> Vec<&dyn Action<T>>;

    /// Gets the id of this social practice.
    fn get_id(&self) -> u32;

    /// Gets the entity for each roles that participate in this practice.
    fn get_roles(&self) -> &HashMap<Role, u32>;

    /// Gets the template of this practice.
    fn get_template(&self) -> &dyn PracticeTemplate<T>;
}
