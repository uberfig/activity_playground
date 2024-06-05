use actix_web::{
    error::ErrorBadRequest,
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};
use config::Config;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::db::DbConn;

// #[derive(Serialize, Deserialize, Debug)]
// pub enum WebfingerParseResult {
//     InvalidStart,
//     MissingUsername,
//     MissingDomain,
// }
#[derive(Serialize, Deserialize, Debug)]
pub struct WebfingerQuery {
    pub has_prefix: bool,
    pub preferred_username: Option<String>,
    pub domain: Option<String>,
}

impl WebfingerQuery {
    fn parse_query(input: String) -> Self {
        let resource = input.strip_prefix("acct:");

        let has_prefix;

        let resource = match resource {
            Some(x) => {
                has_prefix = true;
                x
            }
            None => {
                has_prefix = false;
                &input
            }
        };

        let mut vals = resource.split('@');
        let preferred_username = vals.next();
        let domain = vals.next();
        match preferred_username {
            Some(uname) => {
                if let Some(d) = domain {
                    WebfingerQuery {
                        has_prefix,
                        preferred_username: Some(uname.to_string()),
                        domain: Some(d.to_string()),
                    }
                } else {
                    WebfingerQuery {
                        has_prefix,
                        preferred_username: Some(uname.to_string()),
                        domain: None,
                    }
                }
            }
            None => WebfingerQuery {
                has_prefix,
                preferred_username: None,
                domain: None,
            },
        }
    }
}

#[derive(Deserialize, Debug)]
struct Info {
    resource: String,
}

#[get("/.well-known/webfinger")]
async fn webfinger(
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
    info: web::Query<Info>,
) -> Result<HttpResponse> {
    let resource = info.into_inner().resource;
    // let state = &state.;
    let result = WebfingerQuery::parse_query(resource);
    
    if let Some(x) = result.domain {
        if !x.eq_ignore_ascii_case(&state.instance_domain) {
            return Err(ErrorBadRequest("not from this domain"))
        }
    }
    // let query = match result {
    //     Ok(x) => x,
    //     Err(x) => match x {
    //         WebfingerParseResult::InvalidStart => {
    //             return Err(ErrorBadRequest("query does not start with acct:"))
    //         }
    //         WebfingerParseResult::MissingUsername => {
    //             return Err(ErrorBadRequest("query missing PreferredUsername"))
    //         }
    //         WebfingerParseResult::MissingDomain => {
    //             return Err(ErrorBadRequest("query missing domain"))
    //         }
    //     },
    // };

    // Ok(HttpResponse::Ok().json(r#"{"test": "hello")"#))
    Ok(HttpResponse::Ok()
        .content_type("application/jrd+json; charset=utf-8")
        .body(r#"{"test": "hello"}"#))

    // todo!()
}
