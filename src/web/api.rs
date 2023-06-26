use crate::data::query::RevocationStatus;
use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::service::ask::{GetUser, UpdateUser};
use crate::ServiceError;
// use base64::engine::{general_purpose, GeneralPurpose};
// use base64::Engine;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::Responder;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub const API_KEY_HEADER: &str = "x-api-key";

#[derive(Responder, Debug, thiserror::Error, Serialize)]
pub enum ApiKeyError {
    #[error("API key not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(String),
    #[error("invalid API key format")]
    #[response(status = 400, content_type = "json")]
    DecodeError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey(Vec<u8>);

impl ApiKey {
    pub fn to_base64(&self) -> String {
        base64::encode(self.0.as_slice())
        // Engine::encode(general_purpose::GeneralPurpose, self.0.as_slice())
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl Default for ApiKey {
    fn default() -> Self {
        let key = (0..16).map(|_| rand::random::<u8>()).collect();
        Self(key)
    }
}

impl FromStr for ApiKey {
    type Err = ApiKeyError;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        base64::decode(key)
            .map(ApiKey)
            .map_err(|e| Self::Err::DecodeError(e.to_string()))
    }
}

#[derive(Debug, Responder, thiserror::Error)]
pub enum ApiError {
    #[error("api not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(Json<String>),
    #[error("server error")]
    #[response(status = 500, content_type = "json")]
    Server(Json<String>),
    #[error("client error")]
    #[response(status = 401, content_type = "json")]
    User(Json<String>),
    #[error("duplicate user error")]
    #[response(status = 409, content_type = "json")]
    DuplicateUser(Json<String>),
    #[error("key error")]
    #[response(status = 400, content_type = "json")]
    KeyError(Json<ApiKeyError>),
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::User(e) => Self::User(Json(format!("usr parsing error: {}", e))),
            ServiceError::NotFound => Self::NotFound(Json("invalid user detail".to_owned())),
            ServiceError::Data(e) => {
                println!("{}", e);
                if e.to_string().contains("UNIQUE constraint failed") {
                    Self::DuplicateUser(Json("User already registered".to_owned()))
                } else {
                    Self::Server(Json("a server error occured".to_owned()))
                }
            }
            ServiceError::PermissionError(msg) => Self::User(Json(msg)),
            ServiceError::InvalidDetail => {
                Self::NotFound(Json(String::from("invalid user detail")))
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn server_error() -> Outcome<ApiKey, ApiError> {
            Outcome::Failure((
                Status::InternalServerError,
                ApiError::Server(Json("server error".to_string())),
            ))
        }

        fn key_error(e: ApiKeyError) -> Outcome<ApiKey, ApiError> {
            Outcome::Failure((Status::BadRequest, ApiError::KeyError(Json(e))))
        }

        match req.headers().get_one(API_KEY_HEADER) {
            None => key_error(ApiKeyError::NotFound("API key not found".to_string())),
            Some(key) => {
                let db = match req.guard::<&State<AppDatabase>>().await {
                    Outcome::Success(db) => db,
                    _ => return server_error(),
                };

                let api_key = match ApiKey::from_str(key) {
                    Ok(key) => key,
                    Err(e) => return key_error(e),
                };

                match action::api_key_is_valid(api_key.clone(), db.get_pool()).await {
                    Ok(valid) if valid => Outcome::Success(api_key),
                    Ok(valid) if !valid => {
                        key_error(ApiKeyError::NotFound("API key not found".to_string()))
                    }
                    _ => server_error(),
                }
            }
        }
    }
}

#[rocket::get("/key")]
pub async fn new_api_key(database: &State<AppDatabase>) -> Result<Json<&str>, ApiError> {
    let api_key = action::generate_api_key(database.get_pool()).await?;
    println!("API Key: {}", api_key.to_base64());
    Ok(Json("Api key generated. See logs for details."))
}

#[rocket::get("/logout")]
pub async fn revoke_api_key(
    database: &State<AppDatabase>,
    api_key: ApiKey,
) -> Result<Json<&str>, ApiError> {
    let res = action::revoke_api_key(api_key, database.get_pool()).await?;

    match res {
        RevocationStatus::Revoked => Ok(Json("logout successful")),
        RevocationStatus::NotFound => Err(ApiError::NotFound(Json("invalid request".to_string()))),
    }
}

#[rocket::post("/login", data = "<req>")]
pub async fn get_user(
    req: Json<service::ask::GetUser>,
    database: &State<AppDatabase>,
    _api_key: ApiKey,
) -> Result<Json<crate::User>, ApiError> {
    // use crate::domain::user::field::password;

    let req_body = service::ask::GetUser {
        email: req.email.clone().into(),
        password: req.password.clone().into(),
    };

    let user = action::get_user(req.into_inner(), database.get_pool()).await?;

    if req_body.password.is_some()
        && req_body.password.unwrap().into_inner() == user.clone().password.into_inner()
    {
        Ok(Json(user))
    } else {
        Err(ApiError::NotFound(Json("invalid user detail".to_string())))
    }
}

#[rocket::post("/", data = "<req>")]
pub async fn new_user(
    req: Json<service::ask::NewUser>,
    database: &State<AppDatabase>,
    _api_key: ApiKey,
) -> Result<Json<crate::User>, ApiError> {
    let user = action::new_user(req.into_inner(), database.get_pool()).await?;

    Ok(Json(user))
}

#[rocket::patch("/", data = "<req>")]
pub async fn update_user(
    req: Json<service::ask::UpdateUser>,
    database: &State<AppDatabase>,
    _api_key: ApiKey,
) -> Result<Json<crate::User>, ApiError> {
    let user = action::get_user(
        GetUser {
            email: req.clone().email.clone(),
            password: None,
        },
        database.get_pool(),
    )
    .await?;

    let update_req = UpdateUser {
        email: req.email.clone().into_inner().as_str().into(),
        name: match req.name.clone().is_some() {
            true => req.name.clone(),
            false => Some(user.name),
        },
        password: match req.password.clone().is_some() {
            true => req.password.clone(),
            false => Some(user.password),
        },
    };

    let user = action::update_user(update_req, database.get_pool()).await?;

    Ok(Json(user))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_user, new_user, update_user, new_api_key, revoke_api_key]
}

pub mod catcher {
    use rocket::serde::json::Json;
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req: &Request) -> Json<&'static str> {
        eprintln!("General error: {req:?}");
        Json("something went wrong...")
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> Json<&'static str> {
        eprintln!("Internal error: {req:?}");
        Json("internal server error")
    }

    #[catch(404)]
    fn not_found() -> Json<&'static str> {
        Json("404")
    }

    #[catch(401)]
    fn request_error() -> Json<&'static str> {
        Json("request error")
    }

    #[catch(400)]
    fn missing_api_key() -> Json<&'static str> {
        Json("API key missing/invalid")
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![
            default,
            internal_error,
            not_found,
            request_error,
            missing_api_key
        ]
    }
}
