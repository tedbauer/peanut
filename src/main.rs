use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
    Router,
};
use chrono::Local;
use serde::{Deserialize, Serialize};

use sqlite::Connection;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec::Vec;
use tera::Tera;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    title: String,
    content: String,
    date: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NoteParams {
    title: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WriteCardParams {
    date: String,
}

async fn note(
    Query(params): Query<NoteParams>,
    State(state): State<Arc<Mutex<Connection>>>,
) -> impl IntoResponse {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();

    let query = format!("SELECT * FROM users WHERE title = '{}'", params.title);
    let query2 = format!("SELECT * FROM users WHERE title = '{}'", params.title);
    let conn: Arc<Mutex<Connection>> = state.clone();
    let conn2 = conn.lock().unwrap();

    let content = conn2
        .prepare(query)
        .unwrap()
        .into_iter()
        .into_iter()
        .map(|row| row.unwrap().read::<&str, _>("content").to_string())
        .collect::<Vec<_>>()
        .join("");

    let date = conn2
        .prepare(query2)
        .unwrap()
        .into_iter()
        .into_iter()
        .map(|row| row.unwrap().read::<&str, _>("date").to_string())
        .collect::<Vec<_>>()
        .join("");

    context.insert("content", &content);
    context.insert("title", &params.title);
    context.insert("date", &date);
    Html(tera.render("card.html.tera", &context).unwrap())
}

async fn hello(State(state): State<Arc<Mutex<Connection>>>) -> impl IntoResponse {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();

    let query = "SELECT * FROM users";
    let conn: Arc<Mutex<Connection>> = state.clone();
    let conn2 = conn.lock().unwrap();

    let titles = conn2
        .prepare(query)
        .unwrap()
        .into_iter()
        .into_iter()
        .map(|row| row.unwrap().read::<&str, _>("title").to_string())
        .map(|title| format!("<li><a href=\"/note?title={}\">{}</a></li>", title, title))
        .collect::<Vec<_>>()
        .join("");

    context.insert("notes", &titles);
    Html(tera.render("index.html.tera", &context).unwrap())
}

async fn put_card(
    Query(params): Query<Params>,
    State(state): State<Arc<Mutex<Connection>>>,
) -> impl IntoResponse {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();

    let query = format!(
        "INSERT INTO users VALUES ('{}', '{}', '{}')",
        params.title, params.content, params.date
    );
    let conn: Arc<Mutex<Connection>> = state.clone();
    let conn2 = conn.lock().unwrap();
    conn2.execute(query).unwrap();
    Redirect::to("/")
}

async fn new_card(
) -> impl IntoResponse {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();
    context.insert("date", &format!("{}", Local::now().format("%m/%d/%Y")));
    Html(tera.render("write-card.html.tera", &context).unwrap())
}

#[tokio::main]
async fn main() {
    let connection = sqlite::open(":memory:").unwrap();

    let query = "
    CREATE TABLE IF NOT EXISTS users (title TEXT, content TEXT, date TEXT);
";
    connection.execute(query).unwrap();

    println!("we are now here.");

    let query = "SELECT * FROM users";

    let conn = Mutex::new(connection);

    let shared_state = Arc::new(conn);

    let app = Router::new()
        .route("/", get(hello))
        .route("/write-a-card", get(new_card))
        .route("/put-card", get(put_card))
        .route("/note", get(note))
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
