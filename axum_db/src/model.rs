use serde::{Serialize,Deserialize};
use uuid::Uuid;
#[derive(Debug,sqlx::FromRow)]
pub struct Todo{
    pub id :Uuid,
    pub title:String,
    pub completed:bool
}

