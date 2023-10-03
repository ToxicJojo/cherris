use crate::{Role, Square};

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub role: Role,
    pub capture: Option<Role>,
    pub promotion: Option<Role>,
}
