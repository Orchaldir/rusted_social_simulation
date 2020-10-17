use crate::social::action::{Action, MockAction};
use crate::social::practice::role::Role;
use crate::social::practice::PracticeTemplate;
use std::collections::HashMap;

/// A simple implementation of PracticeTemplate.
pub struct SimplePracticeTemplate<T> {
    id: u32,
    name: String,
    role_names: HashMap<Role, String>,
    actions: HashMap<Role, Vec<Box<dyn Action<T>>>>,
}

impl<T> SimplePracticeTemplate<T> {
    pub fn new(
        id: u32,
        name: String,
        role_names: HashMap<Role, String>,
        actions: HashMap<Role, Vec<Box<dyn Action<T>>>>,
    ) -> SimplePracticeTemplate<T> {
        SimplePracticeTemplate {
            id,
            name,
            role_names,
            actions,
        }
    }
}

impl<T> PracticeTemplate<T> for SimplePracticeTemplate<T> {
    /// Gets all actions of a role in this practice template.
    ///
    /// ```
    ///# use rusted_social_simulation::social::practice::role::Role;
    ///# use rusted_social_simulation::social::practice::simple::create_test_template;
    ///# use rusted_social_simulation::social::practice::PracticeTemplate;
    /// let speaker = Role::Character { id: 0 };
    /// let template = create_test_template();
    ///
    /// let actions = template.get_actions(speaker);
    ///
    /// assert_eq!(actions.len(), 2);
    /// assert_eq!(actions.get(0).unwrap().get_name(), "action0");
    /// assert_eq!(actions.get(1).unwrap().get_name(), "action1");
    /// ```
    fn get_actions(&self, role: Role) -> Vec<&dyn Action<T>> {
        if let Some(actions) = self.actions.get(&role) {
            return actions.iter().map(|action| action.as_ref()).collect();
        }

        Vec::new()
    }

    /// Gets the id of this practice template.
    ///
    /// ```
    ///# use rusted_social_simulation::social::practice::simple::create_test_template;
    ///# use rusted_social_simulation::social::practice::PracticeTemplate;
    /// let template = create_test_template();
    ///
    /// assert_eq!(template.get_id(), 42);
    /// ```
    fn get_id(&self) -> u32 {
        self.id
    }

    /// Gets the name of this practice template.
    ///
    /// ```
    ///# use rusted_social_simulation::social::practice::simple::create_test_template;
    ///# use rusted_social_simulation::social::practice::PracticeTemplate;
    /// let template = create_test_template();
    ///
    /// assert_eq!(template.get_name(), "template0");
    /// ```
    fn get_name(&self) -> &str {
        &self.name
    }

    /// Gets all roles that participate in this practice template.
    ///
    /// ```
    ///# use rusted_social_simulation::social::practice::role::Role;
    ///# use rusted_social_simulation::social::practice::simple::create_test_template;
    ///# use rusted_social_simulation::social::practice::PracticeTemplate;
    /// let speaker = Role::Character { id: 0 };
    /// let listener = Role::Character { id: 1 };
    /// let template = create_test_template();
    ///
    /// let roles = template.get_roles();
    ///
    /// assert_eq!(roles.len(), 2);
    /// assert!(roles.contains(&speaker));
    /// assert!(roles.contains(&listener));
    /// ```
    fn get_roles(&self) -> Vec<Role> {
        self.role_names.iter().map(|(role, _name)| *role).collect()
    }

    /// Gets the name of a role in this practice template.
    ///
    /// # Examples
    ///
    /// ```
    ///# use rusted_social_simulation::social::practice::role::Role;
    ///# use rusted_social_simulation::social::practice::simple::create_test_template;
    ///# use rusted_social_simulation::social::practice::PracticeTemplate;
    /// let speaker = Role::Character { id: 0 };
    /// let template = create_test_template();
    ///
    /// assert_eq!(template.get_role_name(speaker), "Speaker");
    /// ```
    ///
    /// # Panics
    ///
    /// ```should_panic
    ///# use rusted_social_simulation::social::practice::role::Role;
    ///# use rusted_social_simulation::social::practice::simple::create_test_template;
    ///# use rusted_social_simulation::social::practice::PracticeTemplate;
    /// let unknown_role = Role::Character { id: 99 };
    /// let template = create_test_template();
    ///
    /// template.get_role_name(unknown_role);
    /// ```
    fn get_role_name(&self, role: Role) -> &str {
        self.role_names
            .get(&role)
            .unwrap_or_else(|| panic!("Action {} doesn't have the role {}!", self.name, role))
    }
}

pub fn create_test_template() -> SimplePracticeTemplate<u32> {
    let speaker = Role::Character { id: 0 };
    let listener = Role::Character { id: 1 };

    let role_names = hashmap! {
        speaker => "Speaker".to_string(),
        listener => "Listener".to_string(),
    };

    let action0: Box<dyn Action<u32>> = Box::new(MockAction::new("action0".to_string()));
    let action1: Box<dyn Action<u32>> = Box::new(MockAction::new("action1".to_string()));

    let actions = hashmap! {
        speaker => vec![action0, action1],
    };

    SimplePracticeTemplate::new(42, "template0".to_string(), role_names, actions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_actions_for_passive_role() {
        let listener = Role::Character { id: 1 };
        let template = create_test_template();

        assert!(template.get_actions(listener).is_empty());
    }

    #[test]
    fn test_get_actions_for_unknown_role() {
        let unknown_role = Role::Character { id: 99 };
        let template = create_test_template();

        assert!(template.get_actions(unknown_role).is_empty());
    }
}
