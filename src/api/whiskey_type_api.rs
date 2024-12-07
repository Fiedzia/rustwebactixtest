use actix_web::HttpResponse;
use actix_web::{get, post, web, Result};

use diesel::prelude::*;
use diesel::Connection;

use crate::models::whiskey_type_model::*;

use diesel::pg::PgConnection;

use dotenv::dotenv;

fn db_connect() -> PgConnection {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("Database Must Be Set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", &db_url))
}

#[get("/whiskey_type/")]
pub async fn list_whiskey_type() -> Result<HttpResponse> {
    use crate::schema::whiskey_type::dsl::*;

    let mut connection = db_connect();

    let whiskey_types = whiskey_type
        .order_by(name)
        .load::<WhiskeyType>(&mut connection)
        .expect("Error loading data");

    Ok(HttpResponse::Ok().json(WhiskeyTypeResponse {
        status: "ok".to_string(),
        data: whiskey_types,
    }))
}

#[post("/whiskey_type/")]
pub async fn create_whiskey_type(
    whiskey_type_data: web::Json<WhiskeyTypeCreate>,
) -> Result<HttpResponse> {
    use crate::schema::whiskey_type::dsl::*;

    let mut connection = db_connect();

    diesel::insert_into(whiskey_type)
        .values(&whiskey_type_data.into_inner())
        .execute(&mut connection)
        .expect("Error inserting new whiskey type");

    Ok(HttpResponse::Ok().json(StatusResponse {
        status: "ok".to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use serde_json;

    use super::*;

    #[actix_web::test]
    async fn test_index_post() {
        let app = test::init_service(App::new().service(create_whiskey_type)).await;
        let file_content =
            std::fs::read_to_string("tests/mock_data.json").expect("could not reads mock data");

        let whiskey_types: Vec<WhiskeyTypeCreate> =
            serde_json::from_str(file_content.as_str()).expect("JSON was not well-formatted");

        for whiskey_type_ in whiskey_types {
            let req = test::TestRequest::post()
                .set_json(whiskey_type_)
                .uri("/whiskey_type/")
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }
    }
}
