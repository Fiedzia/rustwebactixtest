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
pub async fn create_whiskey_type(whiskey_type_data: web::Json<WhiskeyTypeCreate>) -> Result<HttpResponse> {

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

        let whiskey_types = vec![
            WhiskeyTypeCreate{
                name: "Scotch".to_string(),
                description: "Scotch whisky is the most exported whiskey worldwide, especially to markets like the U.S., France, and emerging markets in Asia.".to_string(),
                annual_sale_in_liters: 1_500_000_000
            },
            WhiskeyTypeCreate{
                name: "Irish".to_string(),
                description: "Irish whiskey has been growing rapidly in recent years, driven by global popularity, especially in the U.S. and Europe.".to_string(),
                annual_sale_in_liters: 100_000_000,
            },
            WhiskeyTypeCreate{
                name: "Bourbon".to_string(),
                description: "Bourbon is the most popular American whiskey, with strong demand both domestically and internationally, especially in markets like Europe and Asia.".to_string(),
                annual_sale_in_liters: 240_000_000,
            },
            WhiskeyTypeCreate{
                name: "Rye".to_string(),
                description: "Rye whiskey has seen a resurgence, particularly in craft cocktails and premium markets.".to_string(),
                annual_sale_in_liters: 30_000_000,
            },
        ];

        for whiskey_type_ in whiskey_types {
            let req = test::TestRequest::post().set_json(
                    whiskey_type_
                ).uri("/whiskey_type/create").to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }
    }
}
