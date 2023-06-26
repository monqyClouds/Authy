use crate::UserError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(addr: &str) -> Result<Self, UserError> {
        if addr.trim().is_empty() {
            Err(UserError::InvalidEmail("empty email".to_string()))
        } else {
            Ok(Self(addr.to_string()))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<&str> for Email {
    fn from(value: &str) -> Self {
        Email(value.to_owned())
    }
}

impl FromStr for Email {
    type Err = UserError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.into()))
    }
}
