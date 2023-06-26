use crate::{domain::user::field::Email, UserError};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    // pub(in crate::data) id: usize,
    pub(in crate::data) name: String,
    pub(in crate::data) email: String,
    pub(in crate::data) password: String,
}

#[derive(Debug)]
pub struct NewUser {
    pub(in crate::data) name: String,
    pub(in crate::data) email: String,
    pub(in crate::data) password: String,
}

impl From<crate::service::ask::NewUser> for NewUser {
    fn from(user: crate::service::ask::NewUser) -> Self {
        Self {
            name: user.name.into_inner(),
            email: user.email.into_inner(),
            password: user.password.into_inner(),
        }
    }
}

pub struct GetUser {
    pub(in crate::data) email: String,
}

impl From<Email> for GetUser {
    fn from(email: Email) -> Self {
        Self {
            email: email.into_inner(),
        }
    }
}

impl From<String> for GetUser {
    fn from(value: String) -> Self {
        Self { email: value }
    }
}

impl From<crate::service::ask::GetUser> for GetUser {
    fn from(req: crate::service::ask::GetUser) -> Self {
        Self {
            email: req.email.into_inner(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateUser {
    pub(in crate::data) email: String,
    pub(in crate::data) name: Option<String>,
    pub(in crate::data) password: Option<String>,
}

// impl Into<GetUser> for

impl TryFrom<User> for crate::domain::user::User {
    type Error = UserError;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        use crate::domain::user::field;
        // use std::str::FromStr;

        Ok(Self {
            name: field::Name::new(&user.name)?,
            email: field::Email::new(&user.email)?,
            password: field::Password::new(&user.password)?,
        })
    }
}

impl From<crate::service::ask::UpdateUser> for UpdateUser {
    fn from(user: crate::service::ask::UpdateUser) -> Self {
        Self {
            email: user.email.into_inner(),
            name: match user.name {
                Some(value) => Some(value.into_inner()),
                None => None,
            },
            password: match user.password {
                Some(value) => Some(value.into_inner()),
                None => None,
            },
        }
    }
}
