use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SessionUser {
    pub email: String,
    pub is_admin: bool,
}
