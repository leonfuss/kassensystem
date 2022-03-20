use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
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
    let request_span = tracing::info_span!(
        "Adding a new user",
            %request_id,
            user_email = %form.email,
            user_matrikelnummer = %form.matrikelnummer,
            user_name = %form.name
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new user datails in the database");

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
    // attacht tracing instruments
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("request_id {}, New users details have been saved", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
