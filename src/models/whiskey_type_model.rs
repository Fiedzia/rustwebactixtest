use serde::{Deserialize, Serialize};
use crate::schema::whiskey_type;
use diesel::prelude::*;

// use this struct to represent json to save a product

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "whiskey_type"]
pub struct WhiskeyType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub annual_sale_in_liters: i64
}


// use this struct to respresent data on response
#[derive(Serialize)]
pub struct WhiskeyTypeResponse {
    pub status: String,
    pub data: Vec<WhiskeyType>
}
