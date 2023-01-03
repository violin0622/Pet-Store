pub mod model;
mod query;
mod schema;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{pg::PgConnection, prelude::*};
use dotenvy::dotenv;
use std::env;

fn new_connection() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&url).expect("Err connect to database {url}")
}

fn new_conn_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Pool::builder()
        .build(ConnectionManager::new(url))
        .expect("Err connect to database {url}")
}

pub use query::DB;
