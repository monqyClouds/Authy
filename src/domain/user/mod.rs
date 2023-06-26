pub mod field;

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("name cannot be empty")]
    EmptyName,

    #[error("invalid email: {0}")]
    InvalidEmail(String),

    #[error("invalid password: {0}")]
    InvalidPassword(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub name: field::Name,
    pub email: field::Email,
    pub password: field::Password,
}
