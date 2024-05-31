use activity_playground::{db::DbAppState, webfinger::webfinger};
use actix_web::{
    error::ErrorBadRequest,
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users/{preferred_username}")]
async fn get_actor(path: web::Path<String>) -> Result<HttpResponse> {
    let preferred_username = path.into_inner();
    // Ok(preferred_username)
    Ok(HttpResponse::Ok().content_type("application/activity+json; charset=utf-8").body(r#"{"test": "hello"}"#))
}

#[get("/@{preferred_username}")]
async fn get_profile_page(conn: Data<DbAppState>, path: web::Path<String>) -> Result<String> {
    let val = sqlx::query!(
        "INSERT INTO internal_users (password, preferredUsername) VALUES ($1, $2)",
        "hi".to_string(),
        "hi".to_string()
    )
    .execute(&conn.db)
    .await;

    let preferred_username = path.into_inner();
    Ok(preferred_username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://ivy:password@localhost/activityfun_dev")
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(DbAppState { db: pool.clone() }))
            .service(hello)
            .service(webfinger)
            .service(get_actor)
            .service(get_profile_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
