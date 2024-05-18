use serde::Deserialize;
use crate::go_cardless::dto::amount_dto::AmountDto;

#[derive(Deserialize, Clone)]
pub struct BalanceDto {
    #[serde(rename(deserialize = "balanceAmount"))]
    pub balance_amount: AmountDto,
    #[serde(rename(deserialize = "balanceType"))]
    pub balance_type: String,
}

impl Default for BalanceDto {
    fn default() -> BalanceDto {
        return BalanceDto {
            balance_amount: AmountDto {
                amount: "0".to_string(),
                currency: "".to_string(),
            },
            balance_type: "".to_string(),
        };
    }
}