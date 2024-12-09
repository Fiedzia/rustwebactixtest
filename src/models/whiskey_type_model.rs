use crate::schema::whiskey_type;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name=whiskey_type)]
pub struct WhiskeyTypeCreate {
    pub name: String,
    pub description: String,
    pub annual_sale_in_liters: i64,
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name=whiskey_type)]
pub struct WhiskeyType {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub annual_sale_in_liters: i64,
}

#[derive(Serialize)]
pub struct WhiskeyTypeResponse {
    pub status: String,
    pub data: Vec<WhiskeyType>,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
}
