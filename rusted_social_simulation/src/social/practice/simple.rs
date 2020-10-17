use crate::social::action::{Action, MockAction};
use crate::social::practice::role::Role;
use crate::social::practice::{Practice, PracticeTemplate};
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
        self.role_names.get(&role).unwrap_or_else(|| {
            panic!(
                "PracticeTemplate '{}' doesn't have the role {}!",
                self.name, role
            )
        })
    }
}

/// A simple implementation of PracticeTemplate.
pub struct SimplePractice<'a, T> {
    id: u32,
    role_to_id_map: HashMap<Role, u32>,
    template: &'a dyn PracticeTemplate<T>,
}

impl<'a, T> SimplePractice<'a, T> {
    pub fn new(
        id: u32,
        role_to_id_map: HashMap<Role, u32>,
        template: &dyn PracticeTemplate<T>,
    ) -> SimplePractice<T> {
        SimplePractice {
            id,
            role_to_id_map,
            template,
        }
    }
}

impl<'a, T> Practice<T> for SimplePractice<'a, T> {
    /// Gets all actions of an entity in this practice.
    ///
    /// ```
    ///# use rusted_social_simulation::social::practice::role::Role;
    ///# use rusted_social_simulation::social::practice::simple::{create_test_template, create_test_practice};
    ///# use rusted_social_simulation::social::practice::{PracticeTemplate, Practice};
    /// let template = create_test_template();
    /// let practice = create_test_practice(&template);
    ///
    /// let actions = practice.get_actions(10);
    ///
    /// assert_eq!(actions.len(), 2);
    /// assert_eq!(actions.get(0).unwrap().get_name(), "action0");
    /// assert_eq!(actions.get(1).unwrap().get_name(), "action1");
    /// ```
    fn get_actions(&self, entity: u32) -> Vec<&dyn Action<T>> {
        let role = self.get_role(entity);
        self.template.get_actions(role)
    }

    /// Gets the id of this social practice.
    ///
    /// ```
    ///# use rusted_social_simulation::social::practice::simple::{create_test_template, create_test_practice};
    ///# use rusted_social_simulation::social::practice::{PracticeTemplate, Practice};
    /// let template = create_test_template();
    /// let practice = create_test_practice(&template);
    ///
    /// assert_eq!(practice.get_id(), 5);
    /// ```
    fn get_id(&self) -> u32 {
        self.id
    }

    /// Gets the role of an entity that participate in this practice.
    ///
    /// # Examples
    ///
    /// ```
    ///# use rusted_social_simulation::social::practice::role::Role;
    ///# use rusted_social_simulation::social::practice::simple::{create_test_template, create_test_practice};
    ///# use rusted_social_simulation::social::practice::{PracticeTemplate, Practice};
    /// let template = create_test_template();
    /// let practice = create_test_practice(&template);
    ///
    /// assert_eq!(practice.get_role(10), Role::Character { id: 0 });
    /// ```
    ///
    /// # Panics
    ///
    /// ```should_panic
    ///# use rusted_social_simulation::social::practice::role::Role;
    ///# use rusted_social_simulation::social::practice::simple::{create_test_template, create_test_practice};
    ///# use rusted_social_simulation::social::practice::{PracticeTemplate, Practice};
    /// let template = create_test_template();
    /// let practice = create_test_practice(&template);
    ///
    /// practice.get_role(99);
    /// ```
    fn get_role(&self, entity: u32) -> Role {
        for (role, id) in &self.role_to_id_map {
            if *id == entity {
                return *role;
            }
        }

        panic!(
            "Practice {} of template '{}' doesn't have the role for entity {}!",
            self.id,
            self.template.get_name(),
            entity
        );
    }

    /// Gets the template of this practice.
    ///
    /// ```
    ///# use rusted_social_simulation::social::practice::simple::{create_test_template, create_test_practice};
    ///# use rusted_social_simulation::social::practice::{PracticeTemplate, Practice};
    /// let template = create_test_template();
    /// let practice = create_test_practice(&template);
    ///
    /// assert_eq!(practice.get_template().get_name(), "template0");
    /// ```
    fn get_template(&self) -> &dyn PracticeTemplate<T> {
        self.template
    }
}

/// Create a SimplePracticeTemplate for testing.
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

/// Create a SimplePractice for testing.
pub fn create_test_practice(template: &dyn PracticeTemplate<u32>) -> SimplePractice<u32> {
    let speaker = Role::Character { id: 0 };
    let listener = Role::Character { id: 1 };

    let role_to_id_map = hashmap! {
        speaker => 10,
        listener => 11,
    };

    SimplePractice::new(5, role_to_id_map, template)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_get_actions_for_passive_role() {
        let listener = Role::Character { id: 1 };
        let template = create_test_template();

        assert!(template.get_actions(listener).is_empty());
    }

    #[test]
    fn test_template_get_actions_for_unknown_role() {
        let unknown_role = Role::Character { id: 99 };
        let template = create_test_template();

        assert!(template.get_actions(unknown_role).is_empty());
    }
}
