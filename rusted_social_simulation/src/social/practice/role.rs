use core::fmt;

/// A role in a social practice.
///
/// Each role is defined by a type & an id:
///
/// ```
///# use rusted_social_simulation::social::practice::role::Role;
/// let role0 = Role::Character{ id: 0 };
/// let role1 = Role::Character{ id: 1 };
/// ```
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Role {
    Character { id: u32 },
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Character { id } => write!(f, "Character({})", id),
        }
    }
}
