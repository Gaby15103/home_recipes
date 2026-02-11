use serde_derive::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterRequestDto {
    #[validate(length(min = 1, max = 20))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 72))]
    pub password: String,

    #[validate(length(min = 1, max = 20))]
    pub first_name: String,

    #[validate(length(min = 1, max = 20))]
    pub last_name: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequestDto {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 72))]
    pub password: String,
}