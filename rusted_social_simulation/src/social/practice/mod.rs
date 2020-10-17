use crate::social::action::Action;
use crate::social::practice::role::Role;

pub mod role;
pub mod simple;

/// A template for a social practice.
///
/// It defines which roles participate in a social practice
/// and which actions are available for each role.
pub trait PracticeTemplate<T> {
    /// Gets all actions of a role in this practice template.
    fn get_actions(&self, role: Role) -> Vec<&dyn Action<T>>;

    /// Gets the id of this practice template.
    fn get_id(&self) -> u32;

    /// Gets the name of this practice template.
    fn get_name(&self) -> &str;

    /// Gets all roles that participate in this practice template.
    fn get_roles(&self) -> Vec<Role>;

    /// Gets the name of a role in this practice template.
    fn get_role_name(&self, role: Role) -> &str;
}

/// A social practice, which is an instance of a template.
///
/// It defines which entity participates as which role.
pub trait Practice<T> {
    /// Gets all actions of an entity in this practice.
    fn get_actions(&self, entity: u32) -> Vec<&dyn Action<T>>;

    /// Gets the id of this social practice.
    fn get_id(&self) -> u32;

    /// Gets the role of an entity that participate in this practice.
    fn get_role(&self, entity: u32) -> Role;

    /// Gets the template of this practice.
    fn get_template(&self) -> &dyn PracticeTemplate<T>;
}
