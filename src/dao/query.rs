use super::model::{NewPet, Pet};
use chrono::NaiveDate;
use diesel::pg::PgConnection;
use diesel::prelude::*;

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

    use super::schema::pets;
    diesel::insert_into(pets::table)
        .values(new_pets)
        .get_results(conn)
        .expect("Err query pets")
}

pub fn find(conn: &mut PgConnection) {
    use super::schema::pets::dsl::*;
    let res = pets.limit(5).load::<Pet>(conn).expect("Err: query pets");
    println!("pets are {res:?}");
}
