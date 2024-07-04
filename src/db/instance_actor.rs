use sqlx::query;

use crate::protocol::instance_actor::InstanceActor;

pub async fn init_instance_actpr(
    conn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    domain: &str,
) -> InstanceActor {
    let instance_actor = query!(r#"SELECT * FROM instance_actor LIMIT 1"#,)
        .fetch_optional(&mut **conn)
        .await;

    let instance_actor = match instance_actor.unwrap() {
        Some(x) => InstanceActor::new(
            openssl::rsa::Rsa::private_key_from_pem(x.private_key.as_bytes()).unwrap(),
            x.public_key_pem,
            domain,
        ),
        None => {
            let rsa = openssl::rsa::Rsa::generate(2048).unwrap();
            let private_key = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();
            let public = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();

            let val = query!(
                r#"INSERT INTO instance_actor 
                    (private_key, public_key_pem)
                VALUES
                    ($1, $2)
                "#,
                &private_key,
                &public,
            )
            .execute(&mut **conn)
            .await;

            val.unwrap();
            InstanceActor::new(
                openssl::rsa::Rsa::private_key_from_pem(private_key.as_bytes()).unwrap(),
                public,
                domain,
            )
        }
    };
    instance_actor
}
