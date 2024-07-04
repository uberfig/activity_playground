use openssl::pkey::{PKey, Private};
use sqlx::query;
use url::Url;

use crate::activitystream_objects::actors::PublicKey;

pub async fn get_private_key<'e, 'c: 'e, E>(
    executor: E,
    userid: i64,
) -> Result<Option<PKey<Private>>, sqlx::Error>
where
    E: 'e + sqlx::PgExecutor<'c>,
{
    let val = query!(
        r#"SELECT * FROM internal_users
            WHERE uid = $1        
        "#,
        userid,
    )
    .fetch_optional(executor)
    .await;

    match val {
        Ok(x) => match x {
            Some(x) => {
                // let key = openssl::rsa::Rsa::private_key_from_pem(key.private_key.as_bytes()).unwrap();
                // let key = PKey::from_rsa(key).unwrap();

                todo!()
            }
            None => Ok(None),
        },
        Err(x) => Err(x),
    }
}
