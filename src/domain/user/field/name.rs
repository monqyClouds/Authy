use serde::{Deserialize, Serialize};

use std::str::FromStr;

use crate::UserError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Result<Self, UserError> {
        if name.trim().is_empty() {
            Err(UserError::EmptyName)
        } else {
            Ok(Self(name.to_string()))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl FromStr for Name {
    type Err = UserError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.into()))
    }
}
