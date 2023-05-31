use actix_web::{HttpServer, App};
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use std::io;


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

    let http_server = HttpServer::new(|| {
        App::new()
    });

    let http_server_binding = http_server.bind(("127.0.0.1", 8000))?
        .workers(4);

    http_server_binding.run().await
}
