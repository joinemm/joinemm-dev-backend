mod discord;
mod handlers;
mod structs;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use kankyo;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Mutex;
use structs::{ApplicationData, Filter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    kankyo::load(false).expect("Failed to load .env file");
    env_logger::init();

    // Start http server
    HttpServer::new( || {
        let filter_object = read_filter("filter.json").unwrap();
        let cors = Cors::default() // <- Construct CORS middleware builder
            .allowed_origin(&filter_object.allowed_origin)
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        let data = web::Data::new(Mutex::new(ApplicationData {
            filter: filter_object
        }));

        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .route("/discord/user", web::get().to(handlers::discord_user_data))
            .route("/discord/user/guilds", web::get().to(handlers::discord_user_guilds))
            .external_resource(
                "discord_auth", 
                "https://discord.com/api/oauth2/authorize?response_type=code&scope=identify&prompt=consent&redirect_uri={redirect}&client_id={client_id}&state={state}"
            )
    })
    .bind(env::var("BIND").unwrap())?
    .run()
    .await
}

fn read_filter<P: AsRef<Path>>(path: P) -> Result<Filter, ()> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let filter_object = serde_json::from_reader(reader).unwrap();
    Ok(filter_object)
}
