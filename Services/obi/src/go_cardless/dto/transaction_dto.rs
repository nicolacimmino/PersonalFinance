use serde::Deserialize;
use crate::go_cardless::dto::account_dto::AccountDto;
use crate::go_cardless::dto::amount_dto::AmountDto;
use crate::go_cardless::dto::balance_dto::BalanceDto;

#[derive(Deserialize)]
pub struct TransactionsDto {
    pub(crate) booked: Vec<TransactionDto>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct TransactionDto {
    #[serde(rename(deserialize = "transactionId"))]
    pub transaction_id: String,
    #[serde(rename(deserialize = "bookingDate"))]
    pub booking_date: String,
    #[serde(rename(deserialize = "valueDate"))]
    pub value_date: Option<String>,
    #[serde(rename(deserialize = "bookingDateTime"))]
    pub booking_date_time: Option<String>,
    #[serde(rename(deserialize = "transactionAmount"))]
    pub transaction_amount: AmountDto,
    #[serde(default)]
    #[serde(rename(deserialize = "creditorName"))]
    pub creditor_name: String,
    #[serde(default)]
    #[serde(rename(deserialize = "debtorName"))]
    pub debtor_name: String,
    #[serde(rename(deserialize = "debtorAccount"))]
    pub debtor_account: Option<AccountDto>,
    #[serde(rename(deserialize = "remittanceInformationUnstructured"))]
    pub remittance_information_unstructured: Option<String>,
    #[serde(rename(deserialize = "remittanceInformationUnstructuredArray"))]
    pub remittance_information_unstructured_array: Option<Vec<String>>,
    #[serde(rename(deserialize = "balanceAfterTransaction"))]
    pub balance_after_transaction: Option<BalanceDto>,
    #[serde(rename(deserialize = "internalTransactionId"))]
    pub internal_transaction_id: String,
}
