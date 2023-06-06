use actix_web::{
    get,
    web::{Data, Json, self},
};
use sqlite::{Connection, State};

use crate::models::Message;

#[get("/posts/{board}")]
pub async fn get_posts(path: web::Path<String>,db: Data<Connection>) -> Json<Vec<Message>> {
    let board = path.into_inner();

    let query = format!("SELECT * FROM messages WHERE board = '{}'", board);
    let mut statement = db.prepare(query).unwrap();

    let mut posts = Vec::new();
    while let State::Row = statement.next().unwrap() {
        posts.push(Message {
            board: statement.read::<String, _>(0).unwrap(),
            thumb_url: statement.read::<String, _>(1).unwrap(),
            content: statement.read::<String, _>(2).unwrap(),
            username: statement.read::<String, _>(3).unwrap(),
            time: statement.read::<String, _>(4).unwrap(),
        });
    }

    Json(posts)
}
