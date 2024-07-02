use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sqlx::query;

use crate::{activitystream_objects::{
    activities::Question, core_types::ActivityStream, object::Object
}, db::actor_utilities::get_ap_actor_by_fedi_id};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DbObject {
    Object(Object),
    Question(Question),
}

impl DbObject {
    pub fn to_activitystream(self) -> ActivityStream {
        match self {
            DbObject::Object(x) => x.to_activitystream(),
            DbObject::Question(_) => todo!(),
        }
    }
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
    NoAttribution,
}

///inserts an object and returns its id
pub async fn create_new_object(
    object: &DbObject,
    mut transaction: sqlx::Transaction<'_, sqlx::Postgres>,
    domain: &str,
) -> Result<i64, InsertErr> {

    let val = match object {
        DbObject::Object(x) => {
            let Some(actor_fedi_id) = x.get_attributed_to() else {
                return Err(InsertErr::NoAttribution);
            };
            let actor = get_ap_actor_by_fedi_id(actor_fedi_id.as_str(), &mut transaction).await;
            let actor_id = actor.ap_user_id.expect("actor fetched from the db did not contain an actor id");

            let published = match x.published {
                Some(x) => x.earliest().timestamp_millis(),
                None => SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
            };

            let internal_type = serde_json::to_string(&InternalTypes::Object).unwrap();
            let activitystream_type = serde_json::to_string(&InternalTypes::Object).unwrap();
            let val = query!(
                r#"INSERT INTO objects 
                            (domain, internal_type, activitystream_type, ap_user_id, published)
                        VALUES
                            ($1, $2, $3, $4, $5)
                        RETURNING obj_id
                        "#,
                domain,
                internal_type,
                &activitystream_type,
                actor_id,
                published
            )
            .fetch_one(&mut *transaction)
            .await;
            
            let obj_id = val.unwrap().obj_id;
            let id_link = format!("https://{}/users/{}/statuses/{}", domain, actor.preferred_username, obj_id);

            let _result = query!(
                "UPDATE objects SET id = $1 WHERE obj_id = $2",
                &id_link,
                obj_id
            )
            .execute(&mut *transaction)
            .await;

            let _result = query!(
                r#"INSERT INTO activity_objects
                            (obj_id, type_field, id, name, attributedTo, content, published)
                        VALUES
                            ($1, $2, $3, $4, $5, $6, $7)
                        "#,
                obj_id,
                activitystream_type,
                &id_link,
                x.name,
                actor_fedi_id.as_str(),
                x.content,
                published
            )
            .execute(&mut *transaction)
            .await;

            obj_id
        }
        DbObject::Question(x) => todo!(),
    };

    transaction.commit().await.unwrap();

    Ok(val)
}

pub async fn get_object_by_db_id(
    obj_id: i64,
    mut transaction: sqlx::Transaction<'_, sqlx::Postgres>,
    domain: &str,
) -> Option<DbObject> {

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
