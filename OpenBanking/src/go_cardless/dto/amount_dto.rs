use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AmountDto {
    pub amount: String,
    pub currency: String,
}