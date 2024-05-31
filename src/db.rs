use actix_web::web::Data;
use sqlx::{Pool, Postgres};

use crate::webfinger::WebfingerQuery;
pub struct DbAppState {
    pub db: Pool<Postgres>,
}

pub async fn get_actor(conn: Data<DbAppState>, query: WebfingerQuery) {

}
