use super::ask;
use crate::data::{query, DatabasePool};
use crate::web::api::ApiKey;
// use crate::domain::user;
use crate::{ServiceError, User};
use std::convert::TryInto;

pub async fn new_user(req: ask::NewUser, pool: &DatabasePool) -> Result<User, ServiceError> {
    let user = query::new_user(req, pool).await?;
    Ok(user.try_into()?)
}

pub async fn get_user(req: ask::GetUser, pool: &DatabasePool) -> Result<User, ServiceError> {
    let _user_password = req.password.clone();
    let user: User = query::get_user(req, pool).await?.try_into()?;
    Ok(user)
}

pub async fn update_user(req: ask::UpdateUser, pool: &DatabasePool) -> Result<User, ServiceError> {
    let user = query::update_user(req, pool).await?;
    Ok(user.try_into()?)
}

pub async fn generate_api_key(pool: &DatabasePool) -> Result<ApiKey, ServiceError> {
    let api_key = ApiKey::default();
    Ok(query::save_api_key(api_key, pool).await?)
}

pub async fn revoke_api_key(
    api_key: ApiKey,
    pool: &DatabasePool,
) -> Result<query::RevocationStatus, ServiceError> {
    Ok(query::revoke_api_key(api_key, pool).await?)
}

pub async fn api_key_is_valid(api_key: ApiKey, pool: &DatabasePool) -> Result<bool, ServiceError> {
    Ok(query::api_key_is_valid(api_key, pool).await?)
}
