
#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn parse(s: String) -> Result<UserEmail, String> {
        unimplemented!()
    }
}

impl AsRef<String> for UserEmail {
    fn as_ref(&self) -> &String {
        &self.0
    } 
}
