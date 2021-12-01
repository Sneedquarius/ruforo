use crate::templates::StatusTemplate;
use actix_web::{get, web, Responder};
use askama_actix::TemplateToResponse;
use ruforo::MyAppData;

#[get("/status")]
pub async fn status_get(my: web::Data<MyAppData<'static>>) -> impl Responder {
    StatusTemplate {
        start_time: &my.cache.start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        logged_in: true,
        username: None,
    }
    .to_response()
}
