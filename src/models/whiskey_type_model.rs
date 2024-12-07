use crate::schema::whiskey_type;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// use this struct to represent json to save a product

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "whiskey_type"]
pub struct WhiskeyTypeCreate {
    pub name: String,
    pub description: String,
    pub annual_sale_in_liters: i64,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct WhiskeyType {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub annual_sale_in_liters: i64,
}

// use this struct to respresent data on response
#[derive(Serialize)]
pub struct WhiskeyTypeResponse {
    pub status: String,
    pub data: Vec<WhiskeyType>,
}
