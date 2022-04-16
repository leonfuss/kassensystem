use actix_web::{post, HttpResponse, web};
use secrecy::Secret;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct LoginForm{
    matrikelnummer: i32,
    password: Secret<String>
}

#[post("/login")]
#[tracing::instrument(
    name = "Authenticate user"
    skip(form,db_pool)
    fields(
        matrikelnummer = %form.matrikelnummer,
    )
)]
pub async fn login(form: web::Form<LoginForm>, db_pool: web::Data<PgPool>) -> HttpResponse {
todo!()
}

#[post("/card")]
#[tracing::instrument(
    name = "Add to card"
)]
pub async fn add_to_card() -> HttpResponse {
    todo!()
}
