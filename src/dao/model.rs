use super::schema::pets;
use crate::pet_store::{self, RegisterPetRequest};
use chrono::NaiveDate;
use diesel::prelude::*;
use std::convert::From;

#[derive(Queryable, Debug)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    pub species: String,
    pub variety: String,
    pub birthday: Option<NaiveDate>,
    pub description: Option<String>,
}

impl From<pet_store::Pet> for Pet {
    fn from(req: pet_store::Pet) -> Self {
        Self {
            id: req.id as i64,
            name: req.name,
            species: req.species,
            variety: req.variety,
            birthday: req.birthday.map(|d| NaiveDate::from(d)),
            description: Some(req.comment),
        }
    }
}

impl From<pet_store::RegisterPetResponse> for Pet {
    fn from(req: pet_store::RegisterPetResponse) -> Self {
        Self {
            id: req.id as i64,
            name: req.name,
            species: req.species,
            variety: req.variety,
            birthday: req.birthday.map(|d| NaiveDate::from(d)),
            description: match req.comment.len() {
                0 => None,
                _ => Some(req.comment),
            },
        }
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name=pets)]
pub struct NewPet {
    pub name: String,
    pub species: String,
    pub variety: String,
    pub birthday: Option<NaiveDate>,
    pub description: Option<String>,
}

impl From<RegisterPetRequest> for NewPet {
    fn from(req: RegisterPetRequest) -> Self {
        NewPet {
            name: req.name,
            species: req.species,
            variety: req.variety,
            birthday: req.birthday.map(|d| NaiveDate::from(d)),
            description: match req.comment.len() {
                0 => None,
                _ => Some(req.comment),
            },
        }
    }
}

impl From<pet_store::Pet> for NewPet {
    fn from(pet: pet_store::Pet) -> Self {
        NewPet {
            name: pet.name,
            species: pet.species,
            variety: pet.variety,
            birthday: pet.birthday.map(|d| d.into()),
            description: match pet.comment.len() {
                0 => None,
                _ => Some(pet.comment),
            },
        }
    }
}
