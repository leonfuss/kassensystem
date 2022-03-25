use crate::domain::UserName;

pub struct NewUser {
    pub email: String,
    pub name: UserName,
    pub matrikelnummer: i32,
}


