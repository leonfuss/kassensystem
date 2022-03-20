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
#[tracing::instrument(
    name = "Adding a new user",
    skip(form,db_pool),
    fields(
        user_email = %form.email,
        user_matrikelnummer = %form.matrikelnummer,
        user_name = %form.name
    )
)]
pub async fn create_user(
    form: web::Form<AddUserFormData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    match insert_subscriber(&db_pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new user details in the Database", skip(form, db_pool))]
async fn insert_subscriber(db_pool: &PgPool, form: &AddUserFormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
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
    .execute(db_pool)
    // attach tracing instruments
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
