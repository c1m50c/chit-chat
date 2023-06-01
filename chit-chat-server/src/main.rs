use tracing_subscriber::fmt::format::FmtSpan;
use tracing::Level;

use actix_web::{HttpServer, App};
use actix::Actor;

use actix_web::web;
use std::io;

pub mod routes;
pub mod server;
pub mod socket;


#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_max_level(Level::DEBUG)
        .with_line_number(true)
        .with_ansi(true)
        .with_file(true)
        .compact()
        .init();

    let chat_server = server::ChatServer::default().start();

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(chat_server.clone()))
            .service(routes::web_socket_route)
    });

    let http_server_binding = http_server.bind(("127.0.0.1", 8000))?
        .workers(4);

    http_server_binding.run().await
}