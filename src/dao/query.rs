#![allow(dead_code)]

use super::{
    model::{NewPet, Pet},
    new_connection,
    schema::pets,
};
use chrono::NaiveDate;
use diesel::{pg::PgConnection, prelude::*};

pub fn insert(conn: &mut PgConnection) -> Vec<Pet> {
    let new_pets = vec![
        NewPet {
            name: "Lucky".to_owned(),
            species: "Dog".to_owned(),
            variety: "Unknown".to_owned(),
            birthday: NaiveDate::from_ymd_opt(2000, 12, 2),
            description: None,
        },
        NewPet {
            name: "Happy".to_owned(),
            species: "Dog".to_owned(),
            variety: "Unknown".to_owned(),
            birthday: NaiveDate::from_ymd_opt(2002, 2, 28),
            description: None,
        },
    ];

    diesel::insert_into(pets::table)
        .values(new_pets)
        .get_results(conn)
        .expect("Err query pets")
}
pub struct DB {}
impl DB {
    pub fn new() -> Self {
        Self {}
    }
    pub fn insert_pet(&self, p: NewPet) -> QueryResult<Pet> {
        let conn = &mut new_connection();
        diesel::insert_into(pets::table)
            .values(vec![p])
            .get_result(conn)
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
}
