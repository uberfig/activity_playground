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
    activities, activitystream_objects::OldActivity, db::db_methods::{create_internal_actor, get_actor_id_from_internal, DbConn}, protocol::verification::generate_digest,
};

#[get("/actor")]
pub async fn get_instance_actor(
    path: web::Path<String>,
    conn: Data<DbConn>,
    request: HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse> {
    todo!()
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

    // let actor = sqlx::query_as!(
    //     DatabaseActor,
    //     "SELECT * FROM activitypub_users WHERE ap_user_id = $1",
    //     id
    // )
    // .fetch_one(&conn.db)
    // .await
    // .unwrap();

    // let actor: OldActor = actor.into();

    // Ok(HttpResponse::Ok()
    //     .content_type("application/activity+json; charset=utf-8")
    //     .body(serde_json::to_string(&actor).unwrap()))

    todo!()
}

#[get("/create_test")]
pub async fn create_test(
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    let x = create_internal_actor(state, conn, "hello".to_string(), "hello".to_string())
        .await
        .unwrap();

    Ok(HttpResponse::Ok().body(format!("{x}")))
}

#[get("/post_test")]
pub async fn post_test(
    // state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    let activity: OldActivity = serde_json::from_str(activities::ACTIVITY).unwrap();

    let val = sqlx::query!(
        "SELECT private_key FROM  internal_users WHERE preferred_username = $1",
        "test"
    )
    .fetch_one(&conn.db)
    .await
    .unwrap();

    let key = openssl::rsa::Rsa::private_key_from_pem(val.private_key.as_bytes()).unwrap();

    post_to_inbox(
        &activity,
        &"https://place.ivytime.gay/users/test".to_string(),
        &"mastodon.social".to_string(),
        &"https://mastodon.social/inbox".to_string(),
        key,
    )
    .await;

    Ok(HttpResponse::Ok().body(""))
}

pub async fn post_to_inbox(
    activity: &OldActivity,
    from_id: &String,
    to_domain: &String,
    to_inbox: &String,
    private_key: Rsa<Private>,
) {
    let keypair = PKey::from_rsa(private_key).unwrap();

    let document = serde_json::to_string(activity).unwrap();
    let date = httpdate::fmt_http_date(SystemTime::now());

    let digest_base64 = &generate_digest(document.as_bytes());

    //string to be signed
    let signed_string = format!("(request-target): post /inbox\nhost: {to_domain}\ndate: {date}\ndigest: SHA-256={digest_base64}");
    let mut signer = openssl::sign::Signer::new(MessageDigest::sha256(), &keypair).unwrap();
    signer.update(signed_string.as_bytes()).unwrap();
    let signature = openssl::base64::encode_block(&signer.sign_to_vec().unwrap());

    // dbg!(&from_id);

    // let header: String = r#"keyId=""#.to_string()
    //     + from_id
    //     + r#"#main-key",headers="(request-target) host date digest",signature=""#
    //     + &signature
    //     + r#"""#;
    let header = format!(
        r#"keyId="{from_id}#main-key",headers="(request-target) host date digest",signature="{signature}""#
    );

    // println!("{}", &header);

    // dbg!(&header);

    let client = reqwest::Client::new();
    let client = client
        .post(to_inbox)
        .header("Host", to_domain)
        .header("Date", date)
        .header("Signature", header)
        .header("Digest", "SHA-256=".to_owned() + digest_base64)
        .body(document);

    dbg!(&client);

    let res = client.send().await;
    dbg!(&res);

    let response = res.unwrap().text().await;

    dbg!(&response);

    if let Ok(x) = response {
        println!("{}", x);
    }
}
