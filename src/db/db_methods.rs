use actix_web::web::Data;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use openssl::rsa::Rsa;
use sqlx::{query, Pool, Postgres};
use url::Url;

use crate::activitystream_objects::actors::{Actor, PublicKey};

pub struct DbConn {
    pub db: Pool<Postgres>,
}

pub async fn get_actor_id_from_internal<'e, 'c: 'e, E>(
    executor: E,
    username: &str,
) -> Result<Option<i64>, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let val = sqlx::query!(
        "SELECT activitypub_actor FROM  internal_users WHERE preferred_username = $1",
        username
    )
    .fetch_optional(executor)
    .await;
    match val {
        Ok(x) => match x {
            Some(x) => Ok(Some(x.activitypub_actor)),
            None => Ok(None),
        },
        Err(x) => Err(x),
    }
}

pub async fn insert_into_ap_users<'e, 'c: 'e, E>(
    executor: E,
    username: &str,
    domain: &str,
    links: &UserLinks,
) -> Result<i64, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let val = query!(
        r#"INSERT INTO activitypub_users
            (id, preferred_username, domain, inbox, outbox, followers, following, liked)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING ap_user_id
        "#,
        links.id,
        username,
        domain,
        links.inbox,
        links.outbox,
        links.followers,
        links.following,
        links.liked,
    )
    .fetch_one(executor)
    .await;

    match val {
        Ok(x) => Ok(x.ap_user_id),
        Err(x) => Err(x),
    }
}

pub async fn insert_into_local_users<'e, 'c: 'e, E>(
    executor: E,
    pass: &str,
    username: &str,
    actor: i64,
    private_key: &str,
) -> Result<i64, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let val = query!(
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
    .fetch_one(executor)
    .await;

    match val {
        Ok(x) => Ok(x.uid),
        Err(x) => Err(x),
    }
}

pub async fn get_private_key<'e, 'c: 'e, E>(
    executor: E,
    userid: &Url,
) -> Result<Option<PublicKey>, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let actor_id = userid.as_str();

    let val = query!(
        r#"SELECT * FROM public_keys
            WHERE owner = $1        
        "#,
        actor_id,
    )
    .fetch_optional(executor)
    .await;

    match val {
        Ok(x) => {
            match x {
                Some(x) => Ok(Some(PublicKey {
                    id: x.id,
                    owner: x.owner,
                    public_key_pem: x.public_key_pem,
                })),
                None => Ok(None),
            }
        },
        Err(x) => Err(x),
    }
}

pub struct UserLinks {
    pub id: String,
    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
    pub liked: String,
}

fn generate_links(domain: &str, uname: &str) -> UserLinks {
    UserLinks {
        id: format!("https://{domain}/users/{uname}"),
        inbox: format!("https://{domain}/users/{uname}/inbox"),
        outbox: format!("https://{domain}/users/{uname}/outbox"),
        followers: format!("https://{domain}/users/{uname}/followers"),
        following: format!("https://{domain}/users/{uname}/following"),
        liked: format!("https://{domain}/users/{uname}/liked"),
    }
}

pub async fn create_internal_actor(
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
    username: String,
    password: String,
) -> Result<i64, ()> {
    let mut transaction = conn.db.begin().await.unwrap();

    //confirm that the username is not taken
    let val = get_actor_id_from_internal(&mut *transaction, "test").await;

    if val.unwrap().is_some() {
        return Err(());
    };

    let links = generate_links(&state.instance_domain, &username);

    let rsa = Rsa::generate(2048).unwrap();

    let private_key = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();

    let public = rsa.public_key_to_pem().unwrap();

    let key_id = format!(
        "https://{}/users/{}#main-key",
        &state.instance_domain, &username
    );
    dbg!(&key_id);

    let x =
        insert_into_ap_users(&mut *transaction, &username, &state.instance_domain, &links).await;

    let _key_id = insert_public_key(
        &mut *transaction,
        &key_id,
        &links.id,
        &String::from_utf8(public).unwrap(),
    )
    .await.unwrap();

    let actor = x.unwrap();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt);

    if password_hash.is_err() {
        return Err(());
    }

    let pass = password_hash.unwrap().to_string();

    let uid =
        insert_into_local_users(&mut *transaction, &pass, &username, actor, &private_key).await;

    transaction.commit().await.unwrap();

    Ok(uid.unwrap())
}

pub async fn create_ap_actor(actor: &Actor, conn: &Data<DbConn>) -> Result<i64, InsertErr> {
    let mut transaction = conn.db.begin().await.unwrap();

    let ap_id = insert_actor_into_ap_users(&mut *transaction, actor).await;

    let ap_id = match ap_id {
        Ok(x) => x,
        Err(x) => {
            transaction.rollback().await.unwrap();
            return Err(x);
        }
    };

    let key_id = insert_actor_public_key(&mut *transaction, actor).await;

    let _key_id = match key_id {
        Ok(x) => x,
        Err(x) => {
            transaction.rollback().await.unwrap();
            return Err(InsertErr::DbErr(x));
        }
    };

    transaction.commit().await.unwrap();

    Ok(ap_id)
}

pub async fn insert_public_key<'e, 'c: 'e, E>(
    executor: E,
    id: &str,
    actor_id: &str,
    public_key_pem: &str,
) -> Result<i64, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let val = query!(
        r#"INSERT INTO public_keys 
            (id, owner, public_key_pem)
        VALUES
            ($1, $2, $3)
        RETURNING pub_key_id
        "#,
        id,
        actor_id,
        public_key_pem
    )
    .fetch_one(executor)
    .await;

    match val {
        Ok(x) => Ok(x.pub_key_id),
        Err(x) => Err(x),
    }
}

/// Note don't forget to insert the actor first
pub async fn insert_actor_public_key<'e, 'c: 'e, E>(
    executor: E,
    actor: &Actor,
) -> Result<i64, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let actor_id = actor.extends_object.id.as_str();

    insert_public_key(
        executor,
        &actor.public_key.id,
        actor_id,
        &actor.public_key.public_key_pem,
    )
    .await
}

pub enum InsertErr {
    NoDomain,
    DbErr(sqlx::Error),
}

/// Note don't forget to insert the public key
pub async fn insert_actor_into_ap_users<'e, 'c: 'e, E>(
    executor: E,
    actor: &Actor,
) -> Result<i64, InsertErr>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let actor_id = actor.extends_object.id.as_str();
    let Some(domain) = actor.extends_object.id.domain() else {
        return Err(InsertErr::NoDomain);
    };

    let val = query!(
        r#"INSERT INTO activitypub_users 
            (id, preferred_username, domain, inbox, outbox, followers, following, liked)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8 )
        RETURNING ap_user_id
        "#,
        actor_id,
        actor.preferred_username,
        domain,
        actor.inbox,
        actor.outbox,
        actor.followers,
        actor.following,
        actor.liked
    )
    .fetch_one(executor)
    .await;

    match val {
        Ok(x) => return Ok(x.ap_user_id),
        Err(x) => return Err(InsertErr::DbErr(x)),
    }
}
