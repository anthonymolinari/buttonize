use actix_web::{web,App,HttpServer};
use actix_web::middleware::Logger;
use actix_embed::Embed;
use rust_embed::RustEmbed;
use env_logger::Env;

mod api;
mod database;
mod models;
mod websocket;

use api::router::{
    health,
    get_device,
    get_device_by_id,
    create_device,
    update_device,
    delete_device,
};
use database::mongo_database::MongoRepo;
use websocket::websocket::connect;

use actix_cors::Cors;

#[derive(RustEmbed)]
#[folder = "src/web/"]
struct Assets;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = web::Data::new(db);
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(web::scope("/api")
                .service(web::scope("/v1")
                    .service(health)
                    .service(get_device)
                    .service(get_device_by_id)
                    .service(create_device)
                    .service(update_device)
                    .service(delete_device)
                )
            )
            .service(web::scope("/socket")
                .service(web::resource("/").route(web::route().to(connect)))
            )
            .service(Embed::new("/", &Assets))
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_origin()
                    .allow_any_method()
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %s"))
    })
    .bind(("0.0.0.0", 8282))?
    .run()
    .await
}
