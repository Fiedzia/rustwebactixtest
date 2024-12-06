mod api;
mod models;
mod schema;

use actix_web:: {
    HttpServer, App, web, HttpResponse, Responder
};

use api::whiskey_type_api::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .app_data(tera)
            .service(create_whiskey_type)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
    
}
