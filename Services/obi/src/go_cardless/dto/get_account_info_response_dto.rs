use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccountInfoResponseDto {
    pub(crate) status: String,
}
