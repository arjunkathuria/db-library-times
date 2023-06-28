// @generated automatically by Diesel CLI.

diesel::table! {
    actor (actor_id) {
        actor_id -> Unsigned<Smallint>,
        #[max_length = 45]
        first_name -> Varchar,
        #[max_length = 45]
        last_name -> Varchar,
        last_update -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    // use super::sql_types::AddressLocationGeometry;

    address (address_id) {
        address_id -> Unsigned<Smallint>,
        #[max_length = 50]
        #[sql_name = "address"]
        address1 -> Varchar,
        #[max_length = 50]
        address2 -> Nullable<Varchar>,
        #[max_length = 20]
        district -> Varchar,
        city_id -> Unsigned<Smallint>,
        #[max_length = 10]
        postal_code -> Nullable<Varchar>,
        #[max_length = 20]
        phone -> Varchar,
        last_update -> Timestamp,
    }
}

diesel::table! {
    category (category_id) {
        category_id -> Unsigned<Tinyint>,
        #[max_length = 25]
        name -> Varchar,
        last_update -> Timestamp,
    }
}

diesel::table! {
    city (city_id) {
        city_id -> Unsigned<Smallint>,
        #[max_length = 50]
        #[sql_name = "city"]
        city1 -> Varchar,
        country_id -> Unsigned<Smallint>,
        last_update -> Timestamp,
    }
}

diesel::table! {
    country (country_id) {
        country_id -> Unsigned<Smallint>,
        #[max_length = 50]
        #[sql_name = "country"]
        country1 -> Varchar,
        last_update -> Timestamp,
    }
}

diesel::table! {
    customer (customer_id) {
        customer_id -> Unsigned<Smallint>,
        store_id -> Unsigned<Tinyint>,
        #[max_length = 45]
        first_name -> Varchar,
        #[max_length = 45]
        last_name -> Varchar,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        address_id -> Unsigned<Smallint>,
        active -> Bool,
        create_date -> Datetime,
        last_update -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    // use super::sql_types::FilmReleaseYearYear;
    // use super::sql_types::FilmRatingEnum;
    // use super::sql_types::FilmSpecialFeaturesSet;

    film (film_id) {
        film_id -> Unsigned<Smallint>,
        #[max_length = 128]
        title -> Text,
        description -> Text,
        // release_year -> Nullable<FilmReleaseYearYear>,
        // language_id -> Unsigned<Tinyint>,
        // original_language_id -> Nullable<Unsigned<Tinyint>>,
        // rental_duration -> Unsigned<Tinyint>,
        rental_rate -> Float,
        //length -> Nullable<Unsigned<Smallint>>,
        // replacement_cost -> Decimal,
        // #[max_length = 5]
        // rating -> Nullable<FilmRatingEnum>,
        // #[max_length = 54]
        // special_features -> Nullable<FilmSpecialFeaturesSet>,
        last_update -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    actor,
    address,
    category,
    city,
    country,
    customer,
    film
);
