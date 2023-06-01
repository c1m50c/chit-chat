use actix_web::{HttpResponse, HttpRequest, Error};
use actix::prelude::*;

use actix_web_actors::ws;
use actix_web::web;

use std::time;

use crate::server;
use crate::socket;


#[actix_web::get("/ws")]
pub async fn web_socket_route(
    request: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    let session = socket::WebSocketSession {
        server: server.get_ref().clone(),
        heartbeat: time::Instant::now(),
        uuid: uuid::Uuid::default(),
        name: None,
    };

    ws::start(session, &request, stream)
}