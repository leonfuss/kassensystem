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
    let request_id = uuid::Uuid::new_v4();
    log::info!(
        "request_id {} - Adding '{}-{}<{}>' as new user",
        request_id,
        form.matrikelnummer,
        form.name,
        form.email
    );
    log::info!("request_id {}, Saving new user details in the databse", request_id);

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
        Ok(_) => {
            log::info!("request_id {}, New users details have been saved", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("request_id {}, Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
