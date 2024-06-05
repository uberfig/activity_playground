use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::webfinger::WebfingerQuery;
pub struct DbConn {
    pub db: Pool<Postgres>,
}

pub async fn get_actor(conn: Data<DbConn>, query: WebfingerQuery) {
    // let val = sqlx::query_as!(
    //     Actor,
    //     "SELECT * FROM internal_users WHERE preferredUsername = $1",
    //     "hi".to_string()
    // )
    // .execute(&conn.db)
    // .await;
}
