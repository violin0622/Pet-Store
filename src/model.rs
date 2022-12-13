use crate::schema::pets;
use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    pub species: String,
    pub variety: String,
    pub birthday: Option<NaiveDate>,
    pub description: Option<String>,
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
