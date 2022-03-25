use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{UserName, NewUser};

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
    let user_name = match UserName::parse(form.0.name) {
        Ok(u) => u,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let new_user = NewUser {
        email: form.0.email,
        matrikelnummer: form.0.matrikelnummer,
        name: user_name,
    };

    match insert_user(&db_pool, &new_user).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new user details in the Database", skip(user, db_pool))]
async fn insert_user(db_pool: &PgPool, user: &NewUser) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, matrikelnummer, name, created_at)   
        VALUES($1, $2, $3, $4, $5)
    "#,
        Uuid::new_v4(),
        user.email,
        user.matrikelnummer,
        user.name.as_ref(),
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
