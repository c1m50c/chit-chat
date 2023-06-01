use actix_web_actors::ws;
use actix::prelude::*;

use crate::server::message;
use crate::server;

use std::time;


pub type WebSocketResult = Result<ws::Message, ws::ProtocolError>;


#[derive(Debug)]
pub struct WebSocketSession {
    pub server: Addr<server::ChatServer>,
    pub heartbeat: time::Instant,
    pub name: Option<String>,
    pub uuid: uuid::Uuid,
}


impl WebSocketSession {
    pub const HEARTBEAT_INTERVAL: time::Duration = time::Duration::from_secs(5);
    pub const TIMEOUT: time::Duration = time::Duration::from_secs(10);

    fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Self::HEARTBEAT_INTERVAL, |actor, ctx| {
            if time::Instant::now().duration_since(actor.heartbeat) > Self::TIMEOUT {
                let message = message::DisconnectMessage {
                    uuid: actor.uuid
                };

                actor.server.do_send(message);
                ctx.stop();
                return;
            }
        });
    }
}


impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat(ctx);

        let address = ctx.address();
        let message = message::ConnectMessage {
            address: address.recipient()
        };

        self.server.send(message).into_actor(self)
            .then(|result, actor, ctx| {
                match result {
                    Ok(uuid) => actor.uuid = uuid,
                    Err(_) => ctx.stop(),
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        let message=  message::DisconnectMessage { uuid: self.uuid };
        self.server.do_send(message);

        Running::Stop
    }
}


impl Handler<message::ServerMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: message::ServerMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.message);
    }
}


impl StreamHandler<WebSocketResult> for WebSocketSession {
    fn handle(&mut self, msg: WebSocketResult, ctx: &mut Self::Context) {
        let Ok(message) = msg else { ctx.stop(); return; };

        match message {
            ws::Message::Ping(message) => {
                self.heartbeat = time::Instant::now();
                ctx.pong(&message);
            },

            ws::Message::Pong(_) => {
                self.heartbeat = time::Instant::now();
            },

            ws::Message::Text(text) => {
                let message = message::ClientMessage {
                    message: text.trim().to_string(),
                    uuid: self.uuid,
                };

                self.server.do_send(message);
            },

            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            },

            ws::Message::Continuation(_) => {
                ctx.stop();
            },

            _ => {  }
        }
    }
}