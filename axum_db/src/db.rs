use sqlx::{PgPool,postgres::PgPoolOptions};
use std::env;
use dotenvy::dotenv;

pub async  fn get_db_pool()->PgPool{
    dotenv().ok();
    let db_url=env::var("DATABASE_URL").expect("Missing Database URL");
    PgPoolOptions::new().max_connections(5).connect(&db_url).await.expect("Failed to connect Db")
}