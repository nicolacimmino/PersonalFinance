use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTokenResponseDto {
    pub(crate) access: String,
}