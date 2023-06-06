use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::{
    debug::{index},
    messages::{get_posts, create_post, get_post}, boards::get_boards,
};
use std::path::Path;

mod api;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        let mut create_tables = "";
        if !Path::new("main.db").exists() {
            create_tables = "CREATE TABLE messages (id INTEGER, board TEXT DEFAULT 'all', thumb_url TEXT DEFAULT '', content TEXT, username TEXT DEFAULT 'anonymous', ref_id INTEGER DEFAULT 0, time DATETIME DEFAULT CURRENT_TIMESTAMP);"
        }

        let connection = sqlite::open("main.db").unwrap();
        connection.execute(create_tables).unwrap();

        let data = Data::new(connection);

        let logger = Logger::default();
        App::new().wrap(logger)
        .app_data(data)
        .service(index)
        .service(get_posts)
        .service(create_post)
        .service(get_boards)
        .service(get_post)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
