use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use sqlx::query;

use crate::activitystream_objects::{
    activities::Question,
    actors::Actor,
    object::{self, Object},
};

use super::conn::DbConn;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DbObject {
    Object(Object),
    Question(Question),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InternalTypes {
    Object,
    Question,
}

pub enum InsertErr {
    NoDomain,
    DbErr(sqlx::Error),
    NoPublishDate,
}

///inserts an object and returns its id
pub async fn create_internal_object(
    object: &DbObject,
    // conn: &Data<DbConn>,
    mut transaction: sqlx::Transaction<'_, sqlx::Postgres>,
    domain: &str,
    actor_id: i64,
) -> Result<i64, InsertErr> {
    // let mut transaction: sqlx::Transaction<'_, sqlx::Postgres> = conn.db.begin().await.unwrap();

    let val = match object {
        DbObject::Object(x) => {
            let internal_type = serde_json::to_string(&InternalTypes::Object).unwrap();
            let activitystream_type = serde_json::to_string(&InternalTypes::Object).unwrap();
            let val = query!(
                r#"INSERT INTO objects 
                            (domain, internal_type, activitystream_type, ap_user_id)
                        VALUES
                            ($1, $2, $3, $4)
                        RETURNING obj_id
                        "#,
                domain,
                internal_type,
                &activitystream_type,
                actor_id
            )
            .fetch_one(&mut *transaction)
            .await;

            let id = val.unwrap().obj_id;

            let Some(published) = x.published else {
                return Err(InsertErr::NoPublishDate);
            };
            let published = published.earliest().timestamp_millis();
            let attributed_to = x.get_attributed_to().unwrap().as_str();

            // let val = query!(
            //     r#"INSERT INTO activity_objects
            //                 (obj_id, type_field, id, name, attributedTo, content, published)
            //             VALUES
            //                 ($1, $2, $3, $4, $5, $6, $7)
            //             RETURNING obj_id
            //             "#,
            //     id,
            //     activitystream_type,
            //     actor_id,
            //     x.name,
            //     attributed_to,
            //     x.content,
            //     published
            // )
            // .fetch_one(&mut *transaction)
            // .await;

            id
        }
        DbObject::Question(x) => todo!(),
    };

    // let ap_id = insert_actor_into_ap_users(&mut *transaction, object, domain).await;

    // let ap_id = match ap_id {
    //     Ok(x) => x,
    //     Err(x) => {
    //         transaction.rollback().await.unwrap();
    //         return Err(x);
    //     }
    // };

    // let _key_id = match key_id {
    //     Ok(x) => x,
    //     Err(x) => {
    //         transaction.rollback().await.unwrap();
    //         return Err(InsertErr::DbErr(x));
    //     }
    // };

    transaction.commit().await.unwrap();

    // Ok(ap_id)
    todo!()
}

// /// must be run with a transaction
// pub async fn insert_object<'e, 'c: 'e, E>(
//     executor: E,
//     object: &DbObject,
//     domain: &str,
// ) -> Result<i64, InsertErr>
// where
//     E: 'e + sqlx::PgExecutor<'c>,
// {
//     let val = query!(
//         r#"INSERT INTO objects
//             (preferred_username, domain, inbox, outbox, followers, following, liked)
//         VALUES
//             ($1, $2, $3, $4, $5, $6, $7, $8 )
//         RETURNING ap_user_id
//         "#,
//         actor_id,
//         actor.preferred_username,
//         domain,
//         actor.inbox,
//         actor.outbox,
//         actor.followers,
//         actor.following,
//         actor.liked
//     )
//     .fetch_one(executor)
//     .await;

//     match val {
//         Ok(x) => return Ok(x.ap_user_id),
//         Err(x) => return Err(InsertErr::DbErr(x)),
//     }
// }

// pub async fn get_object_by_db_id(id: i64, conn: &Data<DbConn>) -> Option<DbObject> {
//     let actor = sqlx::query!("SELECT * FROM activitypub_users WHERE ap_user_id = $1", id)
//         .fetch_one(&conn.db)
//         .await
//         .unwrap();
//     // let test = actor.type_field;
//     let type_field: Result<ActorType, _> = serde_json::from_str(&actor.type_field);
//     let type_field = type_field.expect("somehow an invalid actor type got into the db");

//     let object = Object::new(url::Url::parse(&actor.id).unwrap());

//     let public_key = get_actor_public_key(&conn.db, &actor.id).await.unwrap();

//     Actor {
//         type_field,
//         preferred_username: actor.preferred_username,
//         extends_object: object,
//         public_key,
//         inbox: actor.inbox,
//         outbox: actor.outbox,
//         followers: actor.followers,
//         following: actor.following,
//         ap_user_id: Some(actor.ap_user_id),
//         domain: Some(actor.domain),
//         liked: actor.liked,
//     }
// }
