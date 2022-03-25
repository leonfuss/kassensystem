use super::{UserEmail, UserMatrikelnummer};
use crate::domain::UserName;

pub struct NewUser {
    pub email: UserEmail,
    pub matrikelnummer: UserMatrikelnummer,
    pub name: UserName,
}
