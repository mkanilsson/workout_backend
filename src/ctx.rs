use crate::models::{token::Token, user::User};

pub struct Ctx {
    user: User,
    current_token: Token,
}

impl Ctx {
    pub fn new(user: User, current_token: Token) -> Self {
        Self {
            user,
            current_token,
        }
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn token(&self) -> &Token {
        &self.current_token
    }
}
