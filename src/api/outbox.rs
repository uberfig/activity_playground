// use crate::db::DbConn;
// use actix_web::web::Data;
// use serde::{Deserialize, Serialize};
// use sqlx::{Pool, Postgres};

// pub fn create_activity(conn: &Data<DbConn>, user_id: i64) {}

use actix_web::{
    http::Error,
    post,
    web::{self, Data},
    HttpRequest, HttpResponse,
};

use crate::{api::inbox::Inbox, cache_and_fetch::Cache, db::conn::DbConn};

#[post("/outbox")]
pub async fn shared_outbox(
    request: HttpRequest,
    // conn: Data<DbConn>,
    inbox: Data<Inbox>,
    body: web::Bytes,
    cache: Data<Cache>,
    conn: Data<DbConn>,
) -> Result<HttpResponse, Error> {
    todo!()
}

#[post("/users/{preferred_username}/inbox")]
pub async fn private_outbox(
    request: HttpRequest,
    path: web::Path<String>,
    // conn: Data<DbConn>,
    inbox: Data<Inbox>,
    body: web::Bytes,
    cache: Data<Cache>,
    conn: Data<DbConn>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    let preferred_username = path.into_inner();
    todo!()
}
