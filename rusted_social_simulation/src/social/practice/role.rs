/// A role in a social practice.
///
/// Each role is defined by a type & an id:
///
/// ```
///# use rusted_social_simulation::social::practice::role::Role;
/// let role0 = Role::Character{ id: 0 };
/// let role1 = Role::Character{ id: 1 };
/// ```
#[derive(Clone, Copy)]
pub enum Role {
    Character { id: u32 },
}
