use actix_web::{
    error::ErrorNotFound, get, web::{self, Data}, HttpResponse, Result
};

use crate::{activitystream_objects::Actor, db::DbConn};

#[get("/users/{preferred_username}")]
async fn get_actor(
    path: web::Path<String>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    let preferred_username = path.into_inner();


    let val = sqlx::query!(
        "SELECT activitypub_actor FROM  internal_users WHERE preferred_username = $1",
        preferred_username
    )
    .fetch_optional(&conn.db)
    .await;

    let id = match val.unwrap() {
        Some(x) => x.activitypub_actor,
        None => {
            return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
        }
    };

    let actor = sqlx::query_as!(
        Actor,
        "SELECT * FROM activitypub_users WHERE database_id = $1",
        id
    )
    .fetch_one(&conn.db)
    .await
    .unwrap();

    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(serde_json::to_string(&actor).unwrap()))
}
