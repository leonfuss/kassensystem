use std::time::SystemTime;

use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub balance: f32,
    pub date_created: i32,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub balance: f32,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Transaction {
    pub id: i32,
    pub description: String,
    pub date: SystemTime,
    pub authorization: User,
    pub affected: User,
    pub amount: f32
}

#[derive(Debug, Insertable)]
#[table_name="transactions"]
pub struct NewTransaction<'a> {
    pub description: &'a str,
    pub authorization: i32,
    pub affected: i32,
    pub amount: f32
}
#[derive(Debug, Serialize, Deserialize)]
 pub struct UserJson {
    pub name: String,
    pub email: String,
    pub balance: f32
}
