use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccountInfoResponseDto {
    pub(crate) iban: String,
    pub(crate) status: String,
}
