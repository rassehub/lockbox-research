use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use crate::{models::ChatMessage, chat_server::ChatServer};

pub struct WsChatSession {
    username: String,
    server: web::Data<ChatServer>,
}

impl WsChatSession {
    pub fn new(username: String, server: web::Data<ChatServer>) -> Self {
        Self { username, server }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(msg) = serde_json::from_str::<ChatMessage>(&text) {
                    self.server.send_message(msg).await;
                }
            }
            Ok(ws::Message::Close(reason)) => {
                self.server.remove_user(&self.username);
                ctx.close(reason);
            }
            _ => (),
        }
    }
}

pub async fn chat_ws(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<ChatServer>,
    username: web::Query<String>,
) -> Result<HttpResponse, Error> {
    let username = username.into_inner();
    
    // Register user
    let addr = WsChatSession::new(username.clone(), server.clone());
    server.register_user(username, addr.recipient()).await?;

    // Start WebSocket
    ws::start(addr, &req, stream)
}