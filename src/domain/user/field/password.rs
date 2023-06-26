use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::UserError;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Password(String);

impl Password {
    pub fn new(pass: &str) -> Result<Self, UserError> {
        if pass.len() < 5 {
            Err(UserError::InvalidPassword(String::from(
                "password length less than 5",
            )))
        } else {
            Ok(Self(pass.to_string()))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<&str> for Password {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl FromStr for Password {
    type Err = UserError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.into()))
    }
}
