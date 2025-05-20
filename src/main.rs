use actix_web::{App, HttpServer};

use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

mod health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 8000;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_factory = || {
        App::new()
            .service(health::routes())
    };
    
    // Attempt to bind to IPv6, fallback to IPv4 on AddrNotAvailable
    let addr_v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), PORT);
    let server = match HttpServer::new(app_factory).bind_auto_h2c(addr_v6) {
        Ok(srv) => srv,
        Err(err) if err.kind() == ErrorKind::AddrNotAvailable => {
            eprintln!("IPv6 not available, falling back to IPv4");

            let addr_v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), PORT);
            HttpServer::new(app_factory).bind_auto_h2c(addr_v4)?
        }
        Err(err) => return Err(err),
    };

    server.workers(8).run().await
}

