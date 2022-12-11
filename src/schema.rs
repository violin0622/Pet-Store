// @generated automatically by Diesel CLI.

diesel::table! {
    pets (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        species -> Varchar,
        variety -> Varchar,
        birthday -> Nullable<Date>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    species (species) {
        species -> Varchar,
    }
}

diesel::table! {
    species_variety (species, variety) {
        species -> Varchar,
        variety -> Varchar,
    }
}

diesel::joinable!(species_variety -> species (species));

diesel::allow_tables_to_appear_in_same_query!(
    pets,
    species,
    species_variety,
);
