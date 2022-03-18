use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct AddUserFormData {
    name: String,
    email: String,
    matrikelnummer: i32,
}

#[post("/create")]
pub async fn create_user(
    form: web::Form<AddUserFormData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO users (id, email, matrikelnummer, name, created_at)   
        VALUES($1, $2, $3, $4, $5)
    "#,
        Uuid::new_v4(),
        form.email,
        form.matrikelnummer,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
