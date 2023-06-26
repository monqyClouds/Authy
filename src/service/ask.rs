use crate::domain::user::field;
use crate::Email;

// use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUser {
    pub email: Email,
    pub password: Option<field::Password>,
}

impl GetUser {
    pub fn from_raw(email: &str, password: &str) -> Self {
        Self {
            email: email.into(),
            password: Some(password.into()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub email: Email,
    pub name: field::Name,
    pub password: field::Password,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateUser {
    pub email: Email,
    pub name: Option<field::Name>,
    pub password: Option<field::Password>,
}
