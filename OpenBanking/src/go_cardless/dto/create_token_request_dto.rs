use serde::Serialize;

#[derive(Serialize)]
pub struct CreateTokenRequestDto {
    pub(crate) secret_id: String,
    pub(crate) secret_key: String,
}