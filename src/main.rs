use actix_web::{
    error::ErrorBadRequest,
    get, post,
    web::{self},
    App, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
    let _preferred_username = path.into_inner();
    todo!()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(webfinger)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
