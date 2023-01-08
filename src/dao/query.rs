#![allow(dead_code)]

use super::{
    model::{NewPet, Pet},
    new_conn_pool,
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
        let pet = diesel::insert_into(pets::table)
            .values(vec![p])
            .get_result(&mut self.conns.get()?)?;
        Ok(pet)
    }

    pub fn insert_pets(&self, p: Vec<NewPet>) -> Result<Vec<Pet>> {
        let pets = diesel::insert_into(pets::table)
            .values(p)
            .get_results(&mut self.conns.get()?)?;
        Ok(pets)
    }

    pub fn take_pet(&self, id: i64) -> Result<Pet> {
        let pet = pets::table.find(id).first(&mut self.conns.get()?)?;
        Ok(pet)
    }

    pub fn list_pets(&self) -> Result<Vec<Pet>> {
        let pets = pets::table.load(&mut self.conns.get()?)?;
        Ok(pets)
    }

    pub fn delete_pet(&self, id: i64) -> Result<()> {
        diesel::delete(pets::dsl::pets.find(id)).execute(&mut self.conns.get()?)?;
        Ok(())
    }
}
