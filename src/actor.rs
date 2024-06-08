use std::time::SystemTime;

use actix_web::{
    error::ErrorNotFound,
    get,
    http::header,
    web::{self, Data},
    HttpResponse, Result,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::{Padding, Rsa},
};
use sqlx::query;

use crate::{
    activities,
    activitystream_objects::{OldActor, DatabaseActor, OldActivity, PublicKey},
    db::DbConn,
    inbox,
    verification::generate_digest,
};

#[get("/users/{preferred_username}")]
pub async fn get_actor(path: web::Path<String>, conn: Data<DbConn>) -> Result<HttpResponse> {
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
        DatabaseActor,
        "SELECT * FROM activitypub_users WHERE database_id = $1",
        id
    )
    .fetch_one(&conn.db)
    .await
    .unwrap();

    let actor: OldActor = actor.into();

    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(serde_json::to_string(&actor).unwrap()))
}

#[get("/create_test")]
pub async fn create_test(
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    let x = create_internal_actor(state, conn, "test".to_string(), "test".to_string())
        .await
        .unwrap();

    Ok(HttpResponse::Ok().body(format!("{x}")))
}

pub async fn create_internal_actor(
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
    username: String,
    password: String,
) -> Result<i64, ()> {
    let mut transaction = conn.db.begin().await.unwrap();

    //confirm that the username is not taken
    let val = sqlx::query!(
        "SELECT activitypub_actor FROM  internal_users WHERE preferred_username = $1",
        &username
    )
    .fetch_optional(&mut *transaction)
    .await;

    match val.unwrap() {
        Some(_) => return Err(()),
        None => {}
    };

    let tmp_domain = &state.instance_domain;
    let tmp_uname = &username;
    let id = format!("https://{tmp_domain}/users/{tmp_uname}");

    let inbox = format!("https://{tmp_domain}/users/{tmp_uname}/inbox");
    let outbox = format!("https://{tmp_domain}/users/{tmp_uname}/outbox");
    let followers = format!("https://{tmp_domain}/users/{tmp_uname}/followers");
    let following = format!("https://{tmp_domain}/users/{tmp_uname}/following");
    let liked = format!("https://{tmp_domain}/users/{tmp_uname}/liked");

    let rsa = Rsa::generate(2048).unwrap();

    let private_key = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();

    let public = rsa.public_key_to_pem().unwrap();

    let key_id = format!("https://{tmp_domain}/users/{tmp_uname}#main-key");
    let public_key = PublicKey {
        id: key_id,
        owner: id.clone(),
        public_key_pem: String::from_utf8(public).unwrap(),
    };
    let serialized_pub = serde_json::to_string(&public_key).unwrap();

    let x = query!(
        r#"INSERT INTO activitypub_users 
            (id, preferred_username, domain, inbox, outbox, followers, following, liked, public_key)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9 )
        RETURNING database_id
        "#,
        id,
        &username,
        tmp_domain,
        inbox,
        outbox,
        followers,
        following,
        liked,
        serialized_pub
    )
    .fetch_one(&mut *transaction)
    .await;

    let actor = x.unwrap().database_id;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt);

    if password_hash.is_err() {
        return Err(());
    }

    let pass = password_hash.unwrap().to_string();

    let uid = query!(
        r#"INSERT INTO internal_users 
            (password, preferred_username, activitypub_actor, private_key )
        VALUES
            ($1, $2, $3, $4)
        RETURNING uid
        "#,
        pass,
        &username,
        actor,
        private_key
    )
    .fetch_one(&mut *transaction)
    .await;

    let _x = transaction.commit().await.unwrap();

    Ok(uid.unwrap().uid)
}

#[get("/post_test")]
pub async fn post_test(
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    let activity: OldActivity = serde_json::from_str(activities::activity).unwrap();

    let val = sqlx::query!(
        "SELECT private_key FROM  internal_users WHERE preferred_username = $1",
        "test"
    )
    .fetch_one(&conn.db)
    .await
    .unwrap();

    let key = openssl::rsa::Rsa::private_key_from_pem(val.private_key.as_bytes()).unwrap();

    let x = post_to_inbox(
        &activity,
        &"https://place.ivytime.gay/users/test".to_string(),
        &"mastodon.social".to_string(),
        &"https://mastodon.social/inbox".to_string(),
        key,
    )
    .await;

    Ok(HttpResponse::Ok().body(format!("")))
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

    let header: String = r#"keyId=""#.to_string()
        + from_id
        + r#"#main-key",headers="(request-target) host date digest",signature=""#
        + &signature
        + r#"""#;
    // let header = format!(
    //     r#"keyId="{from_id}",headers="(request-target) host date",signature="{signature}""#
    // );

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

    dbg!(response);
}
