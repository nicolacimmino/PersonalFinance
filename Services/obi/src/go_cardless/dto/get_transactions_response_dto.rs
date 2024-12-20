use serde::Deserialize;
use crate::go_cardless::dto::transaction_dto::TransactionsDto;

#[derive(Deserialize)]
pub struct GetTransactionsResponseDto {
    pub(crate) transactions: TransactionsDto,
}