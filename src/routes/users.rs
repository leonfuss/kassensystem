use actix_web::{post, web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct AddUserFormData {
    name: String,
    email: String,
    matrikelnummer: String,
}


#[post("/create")]
pub async fn create_user(_form: web::Form<AddUserFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
