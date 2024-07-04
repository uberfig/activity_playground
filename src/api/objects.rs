use crate::db::{conn::DbConn, objects::get_object_by_db_id};
use actix_web::{
    error::ErrorNotFound,
    get,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};

#[get("/users/{preferred_username}/statuses/{id}")]
pub async fn get_object(
    path: web::Path<(String, i64)>,
    conn: Data<DbConn>,
    request: HttpRequest,
    body: web::Bytes,
    // state: Data<crate::config::Config>,
) -> Result<HttpResponse> {
    println!("getting an object");

    dbg!(request);
    dbg!(&body);

    let (_preferred_username, object_id) = path.into_inner();

    let object = get_object_by_db_id(object_id, conn.db.begin().await.unwrap()).await;

    let object = match object {
        Some(x) => x,
        None => {
            return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
        }
    };
    let object = object.to_activitystream();

    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(serde_json::to_string(&object).unwrap()))
}
