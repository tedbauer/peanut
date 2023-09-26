use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use base64::{decode, encode};
use rand::Rng;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
    Router,
};
extern crate markdown;
use axum_sessions::{
    async_session::MemoryStore, extractors::WritableSession, PersistencePolicy, SessionLayer,
};
use chrono::Local;
use serde::{Deserialize, Serialize};

use sqlite::Connection;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec::Vec;
use tera::Tera;

struct AppState {
    connection: Mutex<Connection>,
    store: MemoryStore,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    title: String,
    content: String,
    date: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LoginParams {
    username: String,
    password: String,
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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DeleteNoteParams {
    title: String,
}

async fn delete_note(
    Query(params): Query<DeleteNoteParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();

    let query = format!("DELETE FROM notes WHERE title = '{}'", params.title,);
    let conn: Arc<AppState> = state.clone();
    let conn2 = conn.connection.lock().unwrap();
    conn2.execute(query).unwrap();
    Redirect::to(&"/")
}

async fn note(
    Query(params): Query<NoteParams>,
    State(state): State<Arc<AppState>>,
    session: WritableSession,
) -> impl IntoResponse {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();

    let query = format!("SELECT * FROM notes WHERE title = '{}'", params.title);
    let query2 = format!("SELECT * FROM notes WHERE title = '{}'", params.title);
    let query3 = "SELECT * FROM notes";
    let conn: Arc<AppState> = state.clone();
    println!("{}", conn.store.count().await);
    let conn2 = conn.connection.lock().unwrap();

    let content_md_encoded = conn2
        .prepare(query)
        .unwrap()
        .into_iter()
        .into_iter()
        .map(|row| row.unwrap().read::<&str, _>("content").to_string())
        .collect::<Vec<_>>()
        .join("");

    let content_md: String = match decode(content_md_encoded.clone()) {
        Ok(c) => String::from_utf8(c).unwrap_or("Invalid content, sorry. 😿".to_string()),
        _ => content_md_encoded,
    };

    let content = markdown::to_html(&content_md);

    let date = conn2
        .prepare(query2)
        .unwrap()
        .into_iter()
        .into_iter()
        .map(|row| row.unwrap().read::<&str, _>("date").to_string())
        .collect::<Vec<_>>()
        .join("");

    let titles = conn2
        .prepare(query3)
        .unwrap()
        .into_iter()
        .into_iter()
        .map(|row| row.unwrap().read::<&str, _>("title").to_string())
        .map(|title| format!("<li><a href=\"/note?title={}\">{}</a></li>", title, title))
        .collect::<Vec<_>>()
        .join("");

    match &session.get::<String>("username") {
        Some(val) => {
            context.insert("username", &val);
            context.insert("logged_in", &1);
        }
        None => {
            context.insert("username", &"");
            context.insert("logged_in", &0);
        }
    }

    // match &session.get::<usize>("foo") {
    //     Some(val) => {
    //         context.insert("content", &val);
    //     }
    //     None => {
    //         context.insert("content", &content);
    //     }
    // }

    context.insert("content", &content);
    context.insert("title", &params.title);
    context.insert("notes", &titles);
    context.insert("date", &date);
    Html(tera.render("card.html.tera", &context).unwrap())
}

async fn login(
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
    Query(params): Query<LoginParams>,
) -> impl IntoResponse {
    let query = format!("SELECT * FROM users WHERE username = '{}'", params.username);

    let conn: Arc<AppState> = state.clone();
    let conn2 = conn.connection.lock().unwrap();

    let pwd = conn2
        .prepare(query)
        .unwrap()
        .into_iter()
        .into_iter()
        .map(|row| row.unwrap().read::<&str, _>("password").to_string())
        .collect::<Vec<_>>()
        .join("");

    if params.password.eq(&pwd) {
        println!("yep!");
        session
            .insert("username", params.username)
            .expect("Could not store the answer.");
    } else {
        println!("nope");
    }
    Redirect::to(&("/hello"))
}

async fn logout(mut session: WritableSession) -> impl IntoResponse {
    session
        .insert("username", "")
        .expect("Could not store the answer.");
    Redirect::to(&("/"))
}

async fn hello(
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
) -> impl IntoResponse {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();

    let query = "SELECT * FROM notes";
    let query2 = "SELECT * FROM notes";
    let conn: Arc<AppState> = state.clone();
    let conn2 = conn.connection.lock().unwrap();

    if session.get::<String>("username").is_none()
        || session.get::<String>("username").unwrap().eq("")
    {
        return Html("Please login.".to_owned());
        println!("nope");
    }
    println!("yes");

    let titles = conn2
        .prepare(query)
        .unwrap()
        .into_iter()
        .into_iter()
        .map(|row| row.unwrap().read::<&str, _>("title").to_string())
        .map(|title| format!("<li><a href=\"/note?title={}\">{}</a></li>", title, title))
        .collect::<Vec<_>>()
        .join("");

    let titles2 = conn2
        .prepare(query2)
        .unwrap()
        .into_iter()
        .into_iter()
        .map(|row| row.unwrap().read::<&str, _>("title").to_string())
        .map(|title| format!("<input style=\"all:unset\" type=\"button\" value=\"{}\" onclick=\"appendToText('{}')\"></input>", title, title))
        .collect::<Vec<_>>()
        .join("<br>");

    match &session.get::<String>("username") {
        Some(val) => {
            context.insert("username", &val);
            context.insert("logged_in", &1);
        }
        None => {
            context.insert("username", &"");
            context.insert("logged_in", &0);
        }
    }

    context.insert("notes", &titles);
    context.insert("notes_drop", &titles2);
    context.insert("date", &format!("{}", Local::now().format("%m/%d/%Y")));
    Html(tera.render("index.html.tera", &context).unwrap())
}

async fn put_card(
    Query(params): Query<Params>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();

    let encoded_content = encode(params.content);

    let query = format!(
        "INSERT INTO notes VALUES ('{}', '{}', '{}')",
        params.title, encoded_content, params.date
    );
    let conn: Arc<AppState> = state.clone();
    let conn2 = conn.connection.lock().unwrap();
    conn2.execute(query).unwrap();
    Redirect::to(&("/note?title=".to_owned() + &params.title))
}

async fn new_card(mut session: WritableSession) -> impl IntoResponse {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();

    if session.get::<String>("username").is_none() {
        return Html("Please login.".to_owned());
    }

    context.insert("date", &format!("{}", Local::now().format("%m/%d/%Y")));
    Html(tera.render("write-card.html.tera", &context).unwrap())
}

async fn home() -> impl IntoResponse {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();
    Html(tera.render("home.html.tera", &context).unwrap())
}

#[tokio::main]
async fn main() {
    let store = async_session::MemoryStore::new();
    let secret = rand::thread_rng().gen::<[u8; 128]>();
    let session_layer = SessionLayer::new(store.clone(), &secret).with_secure(false);

    let connection = sqlite::open("notes.db").unwrap();

    let query = "
    CREATE TABLE IF NOT EXISTS notes (title TEXT, content TEXT, date TEXT);
    CREATE TABLE IF NOT EXISTS users (username TEXT, password TEXT, UNIQUE(username, password));
    INSERT OR IGNORE INTO users VALUES ('tonnu', 'passwd');
";
    connection.execute(query).unwrap();

    println!("we are now here.");

    let query = "SELECT * FROM notes";

    let conn = Mutex::new(connection);

    let shared_state = Arc::new(AppState {
        connection: conn,
        store,
    });

    let app = Router::new()
        .route("/", get(home))
        .route("/write-a-card", get(new_card))
        .route("/put-card", get(put_card))
        .route("/note", get(note))
        .route("/login", get(login))
        .route("/delete", get(delete_note))
        .route("/logout", get(logout))
        .route("/home", get(home))
        .route("/hello", get(hello))
        .layer(session_layer)
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
