// @generated automatically by Diesel CLI.

diesel::table! {
    whiskey_type (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Text,
        annual_sale_in_liters -> Int8,
    }
}
