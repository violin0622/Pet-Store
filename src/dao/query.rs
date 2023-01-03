#![allow(dead_code)]

use super::{
    model::{NewPet, Pet},
    new_conn_pool, new_connection,
    schema::pets,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct DB {
    conns: Pool<ConnectionManager<PgConnection>>,
}
impl DB {
    pub fn new() -> Self {
        Self {
            conns: new_conn_pool(),
        }
    }

    pub fn insert_pet(&self, p: NewPet) -> Result<Pet> {
        let conn = &mut self.conns.get()?;
        let pet = diesel::insert_into(pets::table)
            .values(vec![p])
            // .get_result::<Pet>(conn)?;
            .get_result(conn)?;
        Ok(pet)
        // .map_err(|e| e.into())
    }

    pub fn insert_pets(&self, p: Vec<NewPet>) -> QueryResult<Vec<Pet>> {
        let conn = &mut new_connection();
        diesel::insert_into(pets::table).values(p).get_results(conn)
    }

    pub fn take_pet(&self, id: i64) -> QueryResult<Pet> {
        let conn = &mut new_connection();
        use pets::dsl::pets;
        pets.find(id).first(conn)
    }

    pub fn list_pets(&self) -> QueryResult<Vec<Pet>> {
        let conn = &mut new_connection();
        use pets::dsl::pets;
        pets.load(conn)
    }

    pub fn delete_pet(&self, id: i64) -> QueryResult<usize> {
        use pets::dsl;
        diesel::delete(dsl::pets.find(id)).execute(&mut new_connection())
    }
}
