pub mod model;
mod query;
// TODO: 将 pub 改为 pri
pub mod schema;

use diesel::{pg::PgConnection, prelude::*};
use dotenvy::dotenv;
use std::env;

// TODO: 重构去掉外部模块对该方法的依赖
pub fn new_connection() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&url).expect("Err connect to database {url}")
}

pub use query::DB;
