use std::sync::Mutex;

use activity_playground::{
    activities::{get_activity, get_object},
    activitystream_objects::core_types::ContextWrap,
    api::{
        actor::{create_test, get_actor, post_test},
        inbox::{inspect_inbox, private_inbox, shared_inbox, Inbox},
        webfinger::webfinger,
    },
    cache_and_fetch::Cache,
    config::Config, db::db::DbConn,
};
use actix_web::{
    // error::ErrorBadRequest,
    get,
    web::{self, Data},
    App,
    HttpResponse,
    HttpServer,
    Responder,
    Result,
};
// use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/@{preferred_username}")]
async fn get_profile_page(/*conn: Data<DbConn>, */ path: web::Path<String>) -> Result<String> {
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
    // let test = ActivityStream::Object(Object { context: Context::Array(vec!["hi".to_string(), "hello".to_string()]), id: "hi".to_string(), name: "hi".to_string() });
    // println!("{}", serde_json::to_string(&test).unwrap());
    let deserialized: ContextWrap = serde_json::from_str(
        r#"{"type":"Object","@context":["https://context1.com","https://context2.com"],"id":"hi","name":"hi"}"#,
    )
    .unwrap();
    dbg!(&deserialized);

    let test = serde_json::to_string_pretty(&deserialized).unwrap();

    println!("{test}");
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

    let inbox = Data::new(Inbox {
        inbox: Mutex::new(Vec::new()),
    });

    let cache = Data::new(Cache::new());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(DbConn { db: pool.clone() }))
            .app_data(Data::new(config.to_owned()))
            .app_data(inbox.clone())
            .app_data(cache.clone())
            .service(hello)
            .service(webfinger)
            .service(get_actor)
            .service(get_profile_page)
            .service(get_activity)
            .service(get_object)
            .service(create_test)
            .service(post_test)
            .service(shared_inbox)
            .service(private_inbox)
            .service(inspect_inbox)
    })
    .bind((bind, port))?
    .run()
    .await
}
