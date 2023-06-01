use actix::prelude::*;


#[derive(Debug)]
pub struct ServerMessage {
    pub message: String,
}


impl Message for ServerMessage {
    type Result = ();
}


#[derive(Debug)]
pub struct ClientMessage {
    pub uuid: uuid::Uuid,
    pub message: String,
}


impl Message for ClientMessage {
    type Result = ();
}


#[derive(Debug)]
pub struct DisconnectMessage {
    pub uuid: uuid::Uuid,
}


impl Message for DisconnectMessage {
    type Result = ();
}


#[derive(Debug)]
pub struct ConnectMessage {
    pub address: Recipient<ServerMessage>,
}


impl Message for ConnectMessage {
    type Result = uuid::Uuid;
}