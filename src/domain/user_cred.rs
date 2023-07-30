use crate::domain::UserName;
use crate::domain::{Email, UserPass};
use std::collections::HashMap;

#[derive(serde::Deserialize, Default, Clone)]
pub struct SignupForm {
   pub  name: String,
   pub  email: String,
   pub  password: String,
}

pub struct UserSignupForm {
    pub name: UserName,
    pub email: Email,
    pub password: UserPass,
}

impl TryFrom<SignupForm> for UserSignupForm {
    type Error = HashMap<&'static str, &'static str>;

    fn try_from(value: SignupForm) -> Result<Self, Self::Error> {
        let mut error_map: Self::Error = HashMap::new();

        let name = UserName::parse(value.name);
        let email = Email::parse(value.email);
        let pass = UserPass::parse(value.password);

        if let Err(name_err) = name {
            error_map.insert("name", name_err);
        }
        if let Err(email_err) = email {
            error_map.insert("email", email_err);
        }
        if let Err(pass_err) = pass {
            error_map.insert("password", pass_err);
        }

        if !error_map.is_empty() {
            Err(error_map)
        } else {
            Ok(Self {
                name: name.unwrap(),
                email: email.unwrap(),
                password: pass.unwrap(),
            })
        }
    }
}

#[derive(serde::Deserialize, Default, Clone)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub struct UserLoginForm {
    pub email: Email,
    pub password: UserPass,
}

impl TryFrom<LoginForm> for UserLoginForm {
    type Error = HashMap<&'static str, &'static str>;

    fn try_from(value: LoginForm) -> Result<Self, Self::Error> {
        let mut error_map: Self::Error = HashMap::new();

        let email = Email::parse(value.email);
        let pass = UserPass::parse(value.password);

        if let Err(email_err) = email {
            error_map.insert("email", email_err);
        }
        if let Err(pass_err) = pass {
            error_map.insert("password", pass_err);
        }
        if !error_map.is_empty() {
            Err(error_map)
        } else {
            Ok(Self {
                email: email.unwrap(),
                password: pass.unwrap(),
            })
        }
    }
}
