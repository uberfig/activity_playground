use std::time::SystemTime;

use actix_web::{
    error::ErrorNotFound,
    get,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};

use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Rsa,
};

use crate::{
    activitystream_objects::core_types::ActivityStream,
    api::activities,
    cache_and_fetch::Cache,
    db::{
        account_creation::create_internal_actor, actor_utilities::get_ap_actor_by_db_id,
        conn::DbConn, internal_actor::get_actor_id_from_internal,
    },
    protocol::verification::{generate_digest, post_to_inbox},
};

#[get("/actor")]
pub async fn get_instance_actor(
    cache: Data<Cache>,
    request: HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse> {
    println!("getting the instance actor");
    dbg!(request);
    dbg!(body);

    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(
            serde_json::to_string(&cache.instance_actor.item.actor.clone().to_activitystream())
                .unwrap(),
        ))
    // Ok(HttpResponse::Ok()
    //     .content_type("application/activity+json; charset=utf-8")
    //     .body(serde_json::to_string(&cache.instance_actor.string_rep.as_ref().unwrap()).unwrap()))
}

#[get("/users/{preferred_username}")]
pub async fn get_actor(
    path: web::Path<String>,
    conn: Data<DbConn>,
    request: HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse> {
    println!("getting the actor");

    dbg!(request);
    dbg!(&body);
    dbg!(String::from_utf8(body.to_vec()));

    let preferred_username = path.into_inner();

    let val = get_actor_id_from_internal(&conn.db, &preferred_username).await;

    let id = match val.unwrap() {
        Some(x) => x,
        None => {
            return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
        }
    };

    let actor = get_ap_actor_by_db_id(id, &conn).await;
    let actor = actor.to_activitystream();

    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(serde_json::to_string(&actor).unwrap()))
}

#[get("/create_test/{preferred_username}")]
pub async fn create_test(
    path: web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    let preferred_username = path.into_inner();

    let x = create_internal_actor(state, conn, preferred_username.clone(), preferred_username)
        .await
        .unwrap();

    Ok(HttpResponse::Ok().body(format!("{x}")))
}

// #[get("/post_test")]
// pub async fn post_test(
//     // state: Data<crate::config::Config>,
//     conn: Data<DbConn>,
// ) -> Result<HttpResponse> {
//     let activity: ActivityStream = serde_json::from_str(activities::ACTIVITY).unwrap();

//     let val = sqlx::query!(
//         "SELECT private_key FROM  internal_users WHERE preferred_username = $1",
//         "test"
//     )
//     .fetch_one(&conn.db)
//     .await
//     .unwrap();

//     let key = openssl::rsa::Rsa::private_key_from_pem(val.private_key.as_bytes()).unwrap();
//     let key = PKey::from_rsa(key).unwrap();

//     let activity_str = serde_json::to_string(&activity).unwrap();
//     post_to_inbox(
//         &activity_str,
//         "https://place.ivytime.gay/users/test",
//         "mastodon.social",
//         "https://mastodon.social/inbox",
//         &key,
//     )
//     .await;

//     Ok(HttpResponse::Ok().body(""))
// }
