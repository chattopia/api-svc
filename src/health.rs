use actix_web::{get, HttpResponse, Scope, web};

#[get("")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

pub fn routes() -> Scope {
    web::scope("/health").service(health_check)
}