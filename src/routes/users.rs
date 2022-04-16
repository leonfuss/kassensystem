use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewUser, UserEmail, UserMatrikelnummer, UserName};

#[derive(serde::Deserialize)]
pub struct AddUserFormData {
    name: String,
    email: String,
    matrikelnummer: i32,
}

impl TryFrom<AddUserFormData> for NewUser {
    type Error = String;

    fn try_from(value: AddUserFormData) -> Result<Self, Self::Error> {
        let name = UserName::parse(value.name)?;
        let email = UserEmail::parse(value.email)?;
        let matrikelnummer = UserMatrikelnummer::parse(value.matrikelnummer)?;
        Ok(NewUser {
            name,
            email,
            matrikelnummer,
        })
    }
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
    match insert_user(&db_pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new user details in the Database", skip(form, db_pool))]
async fn insert_user(db_pool: &PgPool, form: &AddUserFormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, matrikelnummer, name, created_at)   
        VALUES($1, $2, $3, $4, $5)
    "#,
        Uuid::new_v4(),
        user.email.as_ref(),
        user.matrikelnummer.as_ref(),
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
