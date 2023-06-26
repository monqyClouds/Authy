pub mod data;
pub mod domain;
pub mod service;
pub mod web;

use data::AppDatabase;
pub use data::{DataError, DatabasePool};
pub use domain::user::field::Email;
pub use domain::user::{User, UserError};
use rocket::{Build, Rocket};
pub use service::ServiceError;

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<AppDatabase>(config.database)
        // .manage::<Maintenance>(config.maintenance)
        .mount("/api/user", web::api::routes())
        .register("/api/user", web::api::catcher::catchers())
}

pub struct RocketConfig {
    pub database: AppDatabase,
    // pub maintenance: Maintenance,
}
