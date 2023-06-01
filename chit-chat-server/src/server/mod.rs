use std::collections::HashMap;
use actix::prelude::*;

pub mod message;


#[derive(Debug, Default)]
pub struct ChatServer {
    connections: HashMap<uuid::Uuid, Recipient<message::ServerMessage>>,
}


impl ChatServer {
    fn send_message(&self, message: &str, skip: Option<uuid::Uuid>) {
        for (uuid, recipient) in &self.connections {
            if Some(uuid) == skip.as_ref() { continue; }

            let message = message::ServerMessage {
                message: message.to_string(),
            };

            recipient.do_send(message);
        }
    }
}


impl Actor for ChatServer {
    type Context = Context<Self>;
}


impl Handler<message::ClientMessage> for ChatServer {
    type Result = ();

    #[tracing::instrument(skip(self, _ctx), level = "trace")]
    fn handle(&mut self, msg: message::ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.send_message(&msg.message, Some(msg.uuid));
    }
}


impl Handler<message::DisconnectMessage> for ChatServer {
    type Result = ();

    #[tracing::instrument(skip(self, _ctx), level = "trace")]
    fn handle(&mut self, msg: message::DisconnectMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.connections.remove(&msg.uuid);
    }
}


impl Handler<message::ConnectMessage> for ChatServer {
    type Result = MessageResult<message::ConnectMessage>;

    #[tracing::instrument(skip(self, _ctx), level = "trace")]
    fn handle(&mut self, msg: message::ConnectMessage, _ctx: &mut Self::Context) -> Self::Result {
        let uuid = uuid::Uuid::new_v4();

        self.connections.insert(uuid, msg.address);
        MessageResult(uuid)
    }
}