// @generated automatically by Diesel CLI.

diesel::table! {
    pets (id) {
        id -> Int8,
        name -> Varchar,
        species -> Varchar,
        variety -> Varchar,
        birthday -> Nullable<Date>,
        description -> Nullable<Text>,
    }
}
