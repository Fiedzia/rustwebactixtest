// @generated automatically by Diesel CLI.

diesel::table! {
    whiskey_type (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        annual_sale_in_liters -> Int8,
    }
}
