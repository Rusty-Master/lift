use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    start_lat: String,
    start_lon: String,
    end_lat: String,
    end_lon: String,
}

pub async fn create(form: web::Form<FormData>) -> HttpResponse {
    dbg!("{:?}", &form);
    HttpResponse::Ok().finish()
}
