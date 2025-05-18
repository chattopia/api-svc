use actix_web::{App, HttpServer};

mod health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(health::routes())
    })
        .bind_auto_h2c(("127.0.0.1", 8000))?
        .workers(8)
        .run()
        .await
}

