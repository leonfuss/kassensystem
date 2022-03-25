use validator::validate_email;

#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn parse(s: String) -> Result<UserEmail, String> {
        match validate_email(&s) {
            true => Ok(Self(s)),
            false => Err(format!("{} is not a valid email", s)),
        }
    }
}

impl AsRef<String> for UserEmail {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claim::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};

    use super::*;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[test]
    fn emtpy_string_is_rejected() {
        let email = "".to_string();
        assert_err!(UserEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "something.stringe.com".to_string();
        assert_err!(UserEmail::parse(email));
    }

    #[test]
    fn email_missing_is_rejected() {
        let email = "@something.com".to_string();
        assert_err!(UserEmail::parse(email));
    }

    #[quickcheck_macros::quickcheck]
    fn vaild_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        UserEmail::parse(valid_email.0).is_ok()
    }
}
