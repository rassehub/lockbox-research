use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix::prelude::*;
use sqlx::PgPool;
use crate::models::ChatMessage;

pub struct ChatServer {
    db_pool: PgPool,
    sessions: Arc<Mutex<HashMap<String, Recipient<ChatMessage>>>>,
}

impl ChatServer {
    pub fn new(pool: PgPool) -> Self {
        Self {
            db_pool: pool,
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn register_user(&self, username: String, addr: Recipient<ChatMessage>) -> Result<(), sqlx::Error> {
        // Store in database
        sqlx::query("INSERT INTO users (username) VALUES ($1) ON CONFLICT DO NOTHING")
            .bind(&username)
            .execute(&self.db_pool)
            .await?;

        // Store in memory
        self.sessions.lock().unwrap().insert(username, addr);
        Ok(())
    }

    pub async fn send_message(&self, msg: ChatMessage) {
        // Check if recipient is online
        let recipient = self.sessions.lock().unwrap().get(&msg.to).cloned();

        match recipient {
            Some(addr) => {
                // User online - send directly
                addr.do_send(msg);
            }
            None => {
                // User offline - store in DB
                let _ = sqlx::query(
                    "INSERT INTO messages (sender, recipient, content) VALUES ($1, $2, $3)"
                )
                .bind(&msg.from)
                .bind(&msg.to)
                .bind(&msg.content)
                .execute(&self.db_pool)
                .await;
            }
        }
    }

    pub fn remove_user(&self, username: &str) {
        self.sessions.lock().unwrap().remove(username);
    }
}