use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sqlx::query;
use url::Url;

use crate::{
    activitystream_objects::{
        activities::Question,
        core_types::{ActivityStream, RangeLinkExtendsObject},
        link::LinkSimpleOrExpanded,
        object::{Object, ObjectType, ObjectWrapper},
    },
    db::actor_utilities::get_ap_actor_by_fedi_id,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DbObject {
    Object(ObjectWrapper),
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

#[derive(Debug)]
pub enum InsertErr {
    NoDomain,
    DbErr(sqlx::Error),
    NoPublishDate,
    NoAttribution,
}

impl std::fmt::Display for InsertErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertErr::NoDomain => write!(f, "NoDomain"),
            InsertErr::DbErr(x) => write!(f, "DbErr: {}", x),
            InsertErr::NoPublishDate => write!(f, "NoPublishDate"),
            InsertErr::NoAttribution => write!(f, "NoAttribution"),
        }
    }
}

///inserts an object and returns its id
pub async fn create_new_object(
    object: &DbObject,
    mut transaction: sqlx::Transaction<'_, sqlx::Postgres>,
    domain: &str,
) -> Result<i64, InsertErr> {
    let val = match object {
        DbObject::Object(obj_wrap) => {
            let Some(actor_fedi_id) = obj_wrap.object.get_attributed_to() else {
                return Err(InsertErr::NoAttribution);
            };
            let actor = get_ap_actor_by_fedi_id(actor_fedi_id.as_str(), &mut transaction).await;
            let actor_id = actor
                .ap_user_id
                .expect("actor fetched from the db did not contain an actor id");

            let published = match obj_wrap.object.published {
                Some(x) => x.earliest().timestamp_millis(),
                None => SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64,
            };

            let internal_type = serde_json::to_string(&InternalTypes::Object).unwrap();
            let activitystream_type = serde_json::to_string(&obj_wrap.type_field).unwrap();
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
            let id_link = format!(
                "https://{}/users/{}/statuses/{}",
                domain, actor.preferred_username, obj_id
            );

            let _result = query!(
                "UPDATE objects SET id = $1 WHERE obj_id = $2",
                &id_link,
                obj_id
            )
            .execute(&mut *transaction)
            .await;

            let reply = match &obj_wrap.object.in_reply_to {
                Some(x) => Some(x.get_id().as_str()),
                None => None,
            };

            let _result = query!(
                r#"INSERT INTO activity_objects
                            (obj_id, type_field, id, name, attributedTo, content, in_reply_to, published)
                        VALUES
                            ($1, $2, $3, $4, $5, $6, $7, $8)
                        "#,
                obj_id,
                activitystream_type,
                &id_link,
                obj_wrap.object.name,
                actor_fedi_id.as_str(),
                obj_wrap.object.content,
                reply,
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
) -> Option<DbObject> {
    let object = query!(r#"SELECT * FROM objects WHERE obj_id = $1"#, obj_id)
        .fetch_optional(&mut *transaction)
        .await;

    let Some(object) = object.unwrap() else {
        return None;
    };

    let deserialized: InternalTypes = serde_json::from_str(&object.internal_type)
        .expect("could not deserialize internal_type from db");

    match deserialized {
        InternalTypes::Object => {
            let object = query!(
                r#"SELECT * FROM activity_objects WHERE obj_id = $1"#,
                obj_id
            )
            .fetch_optional(&mut *transaction)
            .await
            .unwrap()
            .expect("item exists in objects as type object but does not exist in activity_objects");

            let reply = match object.in_reply_to {
                Some(x) => {
                    let url = Url::parse(&x).expect("invalid reply to link stored in db");
                    Some(RangeLinkExtendsObject::Link(Box::new(
                        LinkSimpleOrExpanded::Simple(url),
                    )))
                }
                None => None,
            };

            let obj_type: ObjectType =
                serde_json::from_str(&object.type_field).expect("invalid object type stored in db");

            let output = Object::new(Url::parse(&object.id).expect("invalid url stored in db"))
                .attributed_to_link(Some(
                    Url::parse(&object.attributedto).expect("invalid actor url stored in db"),
                ))
                .name(object.name)
                .content(object.content)
                .in_reply_to(reply)
                .wrap(obj_type);

            Some(DbObject::Object(output))
        }
        InternalTypes::Question => todo!(),
    }
}
