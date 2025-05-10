use serde::{Deserialize, Serialize};
use actix::prelude::*;

#[derive(Message, Serialize, Deserialize, Debug)]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub from: String,
    pub to: String,
    pub content: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub username: String,
}