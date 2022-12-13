tonic::include_proto!("mod");

pub mod model;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn new_connection() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&url).unwrap_or_else(|_| panic!("Err connect to database {url}"))
}
