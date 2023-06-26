use super::model;
use crate::{web::api::ApiKey, DataError, DatabasePool};
use sqlx::Row;

type Result<T> = std::result::Result<T, DataError>;

pub async fn get_user<M: Into<model::GetUser>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::User> {
    let model = model.into();
    let email = model.email.as_str();

    Ok(
        sqlx::query_as!(model::User, "SELECT * FROM user WHERE email = ?", email)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn new_user<M: Into<model::NewUser>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::User> {
    let model = model.into();

    let _ = sqlx::query!(
        r#"INSERT INTO user (
            name, email, password
        ) 
        VALUES (?, ?, ?)"#,
        model.name,
        model.email,
        model.password
    )
    .execute(pool)
    .await?;

    get_user(model.email, pool).await
}

pub async fn update_user<M: Into<model::UpdateUser>>(
    model: M,
    pool: &DatabasePool,
) -> std::result::Result<model::User, DataError> {
    let model = model.into();

    println!("{:#?}", model);

    let _ = sqlx::query!(
        r#"UPDATE user SET
                name = ?,
                password = ?
            WHERE email = ?
        "#,
        model.name,
        model.password,
        model.email
    )
    .execute(pool)
    .await?;

    get_user(model.email, pool).await
}

pub async fn save_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<ApiKey> {
    let bytes = api_key.clone().into_inner();

    sqlx::query!("INSERT INTO api_keys (api_key) VALUES (?)", bytes)
        .execute(pool)
        .await
        .map(|_| ())?;

    Ok(api_key)
}

pub enum RevocationStatus {
    Revoked,
    NotFound,
}

pub async fn revoke_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<RevocationStatus> {
    let bytes = api_key.into_inner();

    Ok(
        sqlx::query!("DELETE FROM api_keys WHERE api_key = ?", bytes)
            .execute(pool)
            .await
            .map(|result| match result.rows_affected() {
                0 => RevocationStatus::NotFound,
                _ => RevocationStatus::Revoked,
            })?,
    )
}

pub async fn api_key_is_valid(api_key: ApiKey, pool: &DatabasePool) -> Result<bool> {
    let bytes = api_key.into_inner();
    Ok(sqlx::query("SELECT api_key FROM api_keys WHERE api_key = ?")
        .bind(bytes)
        .fetch_all(pool)
        .await
        .map(|row| {
            let count = row.len();
            count > 0
        })?)
}
