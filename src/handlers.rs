pub mod user;
pub mod snippet;



use actix_web::Responder;
pub async fn ping() -> impl Responder {
    "OK"
}
