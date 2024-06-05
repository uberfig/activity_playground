use activity_playground::{config::Config, db::DbConn, webfinger::webfinger};
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
    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(r#"{"test": "hello"}"#))
}

#[get("/@{preferred_username}")]
async fn get_profile_page(conn: Data<DbConn>, path: web::Path<String>) -> Result<String> {
    // let val = sqlx::query!(
    //     "INSERT INTO internal_users (password, preferredUsername) VALUES ($1, $2)",
    //     "hi".to_string(),
    //     "hi".to_string()
    // )
    // .execute(&conn.db)
    // .await;

    let preferred_username = path.into_inner();
    Ok(preferred_username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //----------------config file settings----------------

    let settings = config::Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("gater_config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::default())
        .build();

    let settings = match settings {
        Ok(x) => x,
        Err(x) => {
            eprintln!("{:#?}", x);
            return Ok(());
        }
    };

    let config = match settings.try_deserialize::<Config>() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{:#?}", error);
            return Ok(());
        }
    };

    let bind = config.bind_address.clone();
    let port = config.port;

    //-------------database ------------------

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Error building a connection pool");

    //-----------------------------
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(DbConn { db: pool.clone() }))
            .app_data(Data::new(config.to_owned()))
            .service(hello)
            .service(webfinger)
            .service(get_actor)
            .service(get_profile_page)
    })
    .bind((bind, port))?
    .run()
    .await
}
