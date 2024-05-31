use actix_web::{
    error::ErrorBadRequest,
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::db::DbAppState;

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
async fn webfinger(state: Data<DbAppState>, info: web::Query<Info>) -> Result<HttpResponse> {
    let resource = info.into_inner().resource;
    let result = WebfingerQuery::parse_query(resource);
    let query = match result {
        Ok(x) => x,
        Err(x) => match x {
            WebfingerParseResult::InvalidStart => {
                return Err(ErrorBadRequest("query does not start with acct:"))
            }
            WebfingerParseResult::MissingUsername => {
                return Err(ErrorBadRequest("query missing PreferredUsername"))
            }
            WebfingerParseResult::MissingDomain => {
                return Err(ErrorBadRequest("query missing domain"))
            }
        },
    };

    // Ok(HttpResponse::Ok().json(r#"{"test": "hello")"#))
    Ok(HttpResponse::Ok().content_type("application/jrd+json; charset=utf-8").body(r#"{"test": "hello"}"#))

    // todo!()
}
