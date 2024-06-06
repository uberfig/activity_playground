use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::webfinger::WebfingerQuery;
pub struct DbConn {
    pub db: Pool<Postgres>,
}

pub async fn get_private_key(conn: &Data<DbConn>, userid: i64) {}
