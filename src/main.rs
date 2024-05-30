use actix_web::{
    error::ErrorBadRequest,
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebfingerParseResult {
    InvalidStart,
    MissingUsername,
    MissingDomain,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WebfingerQuery {
    pub preferred_username: String,
    pub domain: String,
}

impl WebfingerQuery {
    fn parse_query(resource: String) -> Result<Self, WebfingerParseResult> {
        let resource = resource.strip_prefix("acct:");

        match resource {
            Some(query) => {
                let mut vals = query.split('@');
                let preferred_username = vals.next();
                let domain = vals.next();
                match preferred_username {
                    Some(uname) => {
                        if let Some(d) = domain {
                            Ok(WebfingerQuery {
                                preferred_username: uname.to_string(),
                                domain: d.to_string(),
                            })
                        } else {
                            Err(WebfingerParseResult::MissingDomain)
                        }
                    }
                    None => Err(WebfingerParseResult::MissingUsername),
                }
            }
            None => Err(WebfingerParseResult::InvalidStart),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Info {
    resource: String,
}

#[get("/.well-known/webfinger")]
async fn webfinger(info: web::Query<Info>) -> Result<String> {
    let resource = info.into_inner().resource;
    let result = WebfingerQuery::parse_query(resource);
    match result {
        Ok(x) => Ok(x.preferred_username),
        Err(x) => match x {
            WebfingerParseResult::InvalidStart => {
                Err(ErrorBadRequest("query does not start with acct:"))
            }
            WebfingerParseResult::MissingUsername => {
                Err(ErrorBadRequest("query missing PreferredUsername"))
            }
            WebfingerParseResult::MissingDomain => Err(ErrorBadRequest("query missing domain")),
        },
    }
}

#[get("/users/{preferred_username}")]
async fn get_actor(path: web::Path<String>) -> Result<String> {
    let preferred_username = path.into_inner();
    Ok(preferred_username)
}

#[get("/@{preferred_username}")]
async fn get_profile_page(state: Data<AppState>, path: web::Path<String>) -> Result<String> {
    let val = sqlx::query!(
        "INSERT INTO internal_users (password, preferredUsername) VALUES ($1, $2)",
        "hi".to_string(),
        "hi".to_string()
    )
    .execute(&state.db)
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
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(hello)
            .service(webfinger)
            .service(get_actor)
            .service(get_profile_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
