// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Geometry"))]
    pub struct AddressLocationGeometry;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct FilmRatingEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Year"))]
    pub struct FilmReleaseYearYear;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Set"))]
    pub struct FilmSpecialFeaturesSet;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Mediumint"))]
    pub struct InventoryInventoryIdMediumint;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Mediumint"))]
    pub struct RentalInventoryIdMediumint;
}

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
    use super::sql_types::AddressLocationGeometry;

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
        location -> AddressLocationGeometry,
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
    use super::sql_types::FilmReleaseYearYear;
    use super::sql_types::FilmRatingEnum;
    use super::sql_types::FilmSpecialFeaturesSet;

    film (film_id) {
        film_id -> Unsigned<Smallint>,
        #[max_length = 128]
        title -> Varchar,
        description -> Nullable<Text>,
        release_year -> Nullable<FilmReleaseYearYear>,
        language_id -> Unsigned<Tinyint>,
        original_language_id -> Nullable<Unsigned<Tinyint>>,
        rental_duration -> Unsigned<Tinyint>,
        rental_rate -> Decimal,
        length -> Nullable<Unsigned<Smallint>>,
        replacement_cost -> Decimal,
        #[max_length = 5]
        rating -> Nullable<FilmRatingEnum>,
        #[max_length = 54]
        special_features -> Nullable<FilmSpecialFeaturesSet>,
        last_update -> Timestamp,
    }
}

diesel::table! {
    film_actor (actor_id, film_id) {
        actor_id -> Unsigned<Smallint>,
        film_id -> Unsigned<Smallint>,
        last_update -> Timestamp,
    }
}

diesel::table! {
    film_category (film_id, category_id) {
        film_id -> Unsigned<Smallint>,
        category_id -> Unsigned<Tinyint>,
        last_update -> Timestamp,
    }
}

diesel::table! {
    film_text (film_id) {
        film_id -> Smallint,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::InventoryInventoryIdMediumint;

    inventory (inventory_id) {
        inventory_id -> Unsigned<InventoryInventoryIdMediumint>,
        film_id -> Unsigned<Smallint>,
        store_id -> Unsigned<Tinyint>,
        last_update -> Timestamp,
    }
}

diesel::table! {
    language (language_id) {
        language_id -> Unsigned<Tinyint>,
        #[max_length = 20]
        name -> Char,
        last_update -> Timestamp,
    }
}

diesel::table! {
    payment (payment_id) {
        payment_id -> Unsigned<Smallint>,
        customer_id -> Unsigned<Smallint>,
        staff_id -> Unsigned<Tinyint>,
        rental_id -> Nullable<Integer>,
        amount -> Decimal,
        payment_date -> Datetime,
        last_update -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RentalInventoryIdMediumint;

    rental (rental_id) {
        rental_id -> Integer,
        rental_date -> Datetime,
        inventory_id -> Unsigned<RentalInventoryIdMediumint>,
        customer_id -> Unsigned<Smallint>,
        return_date -> Nullable<Datetime>,
        staff_id -> Unsigned<Tinyint>,
        last_update -> Timestamp,
    }
}

diesel::table! {
    staff (staff_id) {
        staff_id -> Unsigned<Tinyint>,
        #[max_length = 45]
        first_name -> Varchar,
        #[max_length = 45]
        last_name -> Varchar,
        address_id -> Unsigned<Smallint>,
        picture -> Nullable<Blob>,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        store_id -> Unsigned<Tinyint>,
        active -> Bool,
        #[max_length = 16]
        username -> Varchar,
        #[max_length = 40]
        password -> Nullable<Varchar>,
        last_update -> Timestamp,
    }
}

diesel::table! {
    store (store_id) {
        store_id -> Unsigned<Tinyint>,
        manager_staff_id -> Unsigned<Tinyint>,
        address_id -> Unsigned<Smallint>,
        last_update -> Timestamp,
    }
}

diesel::joinable!(address -> city (city_id));
diesel::joinable!(city -> country (country_id));
diesel::joinable!(customer -> address (address_id));
diesel::joinable!(customer -> store (store_id));
diesel::joinable!(film_actor -> actor (actor_id));
diesel::joinable!(film_actor -> film (film_id));
diesel::joinable!(film_category -> category (category_id));
diesel::joinable!(film_category -> film (film_id));
diesel::joinable!(inventory -> film (film_id));
diesel::joinable!(inventory -> store (store_id));
diesel::joinable!(payment -> customer (customer_id));
diesel::joinable!(payment -> rental (rental_id));
diesel::joinable!(payment -> staff (staff_id));
diesel::joinable!(rental -> customer (customer_id));
//diesel::joinable!(rental -> inventory (inventory_id));
diesel::joinable!(rental -> staff (staff_id));
diesel::joinable!(staff -> address (address_id));
diesel::joinable!(store -> address (address_id));

diesel::allow_tables_to_appear_in_same_query!(
    actor,
    address,
    category,
    city,
    country,
    customer,
    film,
    film_actor,
    film_category,
    film_text,
    inventory,
    language,
    payment,
    rental,
    staff,
    store,
);
