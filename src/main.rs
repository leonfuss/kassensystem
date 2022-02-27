#[macro_use]
extern crate diesel;
extern crate rocket;

mod config;
mod models;
mod schema;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().attach(database::DbConnection::fairing())
}

mod database {
    use rocket_sync_db_pools::{database, diesel};

    #[database("database")]
    pub struct DbConnection(diesel::SqliteConnection);
}
