use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, Result};

use std::env;
use diesel::prelude::*;

use crate::models::whiskey_type_model::*;

use diesel::{ Connection, prelude::* };

use diesel::pg::PgConnection;


use dotenv::dotenv;


// use actix_web::{ Data };


fn db_connect() -> PgConnection {


    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("Database Must Be Set");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", &db_url))

}


#[post("/whiskey_type/create")]
pub async fn create_whiskey_type(whiskey_type_data: web::Json<WhiskeyType>) -> Result<HttpResponse> {

    use crate::schema::whiskey_type::dsl::*;

    let mut connection = db_connect();

    diesel::insert_into(whiskey_type)
        .values(&whiskey_type_data.into_inner())
        .execute(& mut connection)
        .expect("Error inserting new whiskey type");

    Ok(HttpResponse::Ok().json("New whiskey type was added"))
}





#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    use super::*;


    #[actix_web::test]
    async fn test_index_post() {
        let app = test::init_service(App::new().service(create_whiskey_type)).await;
        let req = test::TestRequest::post().set_json(
                WhiskeyType{
                    id: "scotch".to_string(),
                    name: "Scotch".to_string(),
                    description: "Scotch whisky is the most exported whiskey worldwide, especially to markets like the U.S., France, and emerging markets in Asia.".to_string(),
                    annual_sale_in_liters: 1_500_000_000
                }
            ).uri("/whiskey_type/create").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
