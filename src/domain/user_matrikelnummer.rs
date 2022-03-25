pub struct UserMatrikelnummer(i32);

impl UserMatrikelnummer {
    pub fn parse(n: i32) -> Result<UserMatrikelnummer, String> {
        Ok(Self(n))
    }
}

impl AsRef<i32> for UserMatrikelnummer {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}
