// use crate::db::DbConn;
// use actix_web::web::Data;
// use serde::{Deserialize, Serialize};
// use sqlx::{Pool, Postgres};

// pub fn create_activity(conn: &Data<DbConn>, user_id: i64) {}

use actix_web::{
    get,
    http::Error,
    post,
    web::{self, Data},
    HttpRequest, HttpResponse,
};
use openssl::pkey::PKey;
use url::Url;

use crate::{
    activitystream_objects::object::{Object, ObjectType},
    api::inbox::Inbox,
    cache_and_fetch::Cache,
    db::{
        conn::DbConn,
        internal_actor::get_actor_id_from_internal,
        objects::{create_new_object, get_object_by_db_id},
        private_key::get_private_key,
    },
    protocol::verification::post_to_inbox,
};

#[post("/users/{preferred_username}/outbox")]
pub async fn create_post(
    request: HttpRequest,
    path: web::Path<String>,
    // conn: Data<DbConn>,
    body: web::Bytes,
    cache: Data<Cache>,
    conn: Data<DbConn>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    let preferred_username = path.into_inner();
    let user_id = format!(
        "https://{}/users/{}",
        &state.instance_domain, &preferred_username
    );

    let Ok(body) = String::from_utf8(body.to_vec()) else {
        return Ok(HttpResponse::BadRequest().body("invalid body"));
    };

    dbg!(&user_id);

    let mut object = Object::new(Url::parse("https://temp.com").unwrap())
        .content(Some(body))
        .attributed_to_link(Some(Url::parse(&user_id).unwrap()))
        .wrap(ObjectType::Note);

    let obj_id = create_new_object(
        &crate::db::objects::DbObject::Object(object),
        conn.db.begin().await.unwrap(),
        &state.instance_domain,
    )
    .await;

    let obj_id = match obj_id {
        Ok(x) => x,
        Err(x) => return Ok(HttpResponse::BadRequest().body(format!("{}", x))),
    };

    let id_link = format!(
        "https://{}/users/{}/statuses/{}",
        &state.instance_domain, preferred_username, obj_id
    );

    let object = get_object_by_db_id(obj_id, conn.db.begin().await.unwrap())
        .await
        .unwrap();

    // let actor_id = get_actor_id_from_internal(&conn.db, &preferred_username).await.unwrap().unwrap();
    // let key = get_private_key(&conn.db, actor_id).await;
    let key = sqlx::query!(
        "SELECT * FROM  internal_users WHERE preferred_username = $1",
        &preferred_username
    )
    .fetch_one(&conn.db)
    .await
    .unwrap();

    let key = openssl::rsa::Rsa::private_key_from_pem(key.private_key.as_bytes()).unwrap();
    let key = PKey::from_rsa(key).unwrap();

    match object {
        crate::db::objects::DbObject::Object(x) => {
            let activity = x.to_activitystream();
            let activity_str = serde_json::to_string(&activity).unwrap();

            post_to_inbox(
                &activity_str,
                &user_id,
                "mastodon.social",
                "https://mastodon.social/inbox",
                &key,
            )
            .await;
            // post_to_inbox(
            //     &activity_str,
            //     &user_id,
            //     "cutie.city",
            //     "https://cutie.city/inbox",
            //     &key,
            // )
            // .await;

            return Ok(HttpResponse::Created().body(format!("{}", activity_str)));
        }
        crate::db::objects::DbObject::Question(_) => todo!(),
    }
}

#[get("/users/{preferred_username}/outbox")]
pub async fn private_outbox(
    request: HttpRequest,
    path: web::Path<String>,
    // conn: Data<DbConn>,
    body: web::Bytes,
    cache: Data<Cache>,
    conn: Data<DbConn>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    let preferred_username = path.into_inner();
    return Ok(HttpResponse::NotFound().body(format!("")))
}
