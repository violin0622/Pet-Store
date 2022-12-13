use chrono::NaiveDate;
use diesel::prelude::*;
use pet_store::model::{NewPet, Pet};
use pet_store::new_connection;

fn main() {
    let conn = &mut new_connection();
    let new_pets = insert(conn);
    println!("inserting new pets: {new_pets:?}");
    query(conn);
}

fn insert(conn: &mut PgConnection) -> Vec<Pet> {
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

    use pet_store::schema::pets;
    diesel::insert_into(pets::table)
        .values(new_pets)
        .get_results(conn)
        .expect("Err query pets")
}

fn query(conn: &mut PgConnection) {
    use pet_store::schema::pets::dsl::*;
    let res = pets.limit(5).load::<Pet>(conn).expect("Err: query pets");
    println!("pets are {res:?}");
}
