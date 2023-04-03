use actix_web::{
    HttpResponse,
    Responder,
    get
};

#[get("/")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}
