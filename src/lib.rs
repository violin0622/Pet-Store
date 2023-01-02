tonic::include_proto!("mod");

pub mod dao;
pub mod svc;

use dao::model;
use std::convert::From;
impl From<model::Pet> for pet_store::RegisterPetResponse {
    fn from(p: model::Pet) -> Self {
        Self {
            id: p.id as u64,
            name: p.name,
            species: p.species,
            variety: p.variety,
            birthday: p.birthday.map(|x| x.into()),
            comment: p.description.unwrap_or("".to_owned()),
        }
    }
}

use chrono::{Datelike, NaiveDate};
impl From<NaiveDate> for google::r#type::Date {
    fn from(d: NaiveDate) -> Self {
        Self {
            year: d.year() as i32,
            month: d.month0() as i32,
            day: d.day() as i32,
        }
    }
}
