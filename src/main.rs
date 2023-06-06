use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::{
    debug::{index, insert_sample_data},
    messages::get_posts,
};
use std::path::Path;

mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        let mut create_tables = "";
        if !Path::new("main.db").exists() {
            create_tables = "CREATE TABLE messages (board TEXT DEFAULT 'all', thumb_url TEXT DEFAULT '', content TEXT, username TEXT DEFAULT 'anonymous', time DATETIME DEFAULT CURRENT_TIMESTAMP);"
        }

        let connection = sqlite::open("main.db").unwrap();
        connection.execute(create_tables).unwrap();

        let data = Data::new(connection);

        let logger = Logger::default();
        App::new().wrap(logger)
        .app_data(data)
        .service(index)
        .service(insert_sample_data)
        .service(get_posts)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
