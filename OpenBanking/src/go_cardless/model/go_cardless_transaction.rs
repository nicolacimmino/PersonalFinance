use crate::go_cardless::dto::prelude::{AccountDto, BalanceDto, TransactionDto};

pub struct GoCardlessTransaction {
    pub transaction_id: String,
    pub booking_date: String,
    pub value_date: String,
    pub transaction_amount_cents: i32,
    pub transaction_amount_currency: String,
    pub creditor_name: String,
    pub debtor_name: String,
    pub debtor_account_iban: String,
    pub remittance_information_unstructured: String,
    pub balance_after_transaction_cents: i32,
    pub balance_after_transaction_type: String,
    pub internal_transaction_id: String,
}

pub trait ConvertsToGoCardlessTransaction {
    fn to_gocardless_transaction(&self) -> GoCardlessTransaction;
}

impl ConvertsToGoCardlessTransaction for &TransactionDto {
    fn to_gocardless_transaction(&self) -> GoCardlessTransaction {
        let value_date = self.value_date.clone().unwrap_or(self.booking_date.to_string());

        let transaction_amount_cents = (self.transaction_amount.amount
            .parse::<f64>()
            .expect(
                format!("transaction_amount: {}", self.transaction_amount.amount).as_str()
            ) * 100f64) as i32;

        let debtor_account_iban = self.debtor_account.clone()
            .unwrap_or(AccountDto { ..Default::default() })
            .iban;

        let remittance_information_unstructured =
            self.remittance_information_unstructured.clone()
                .unwrap_or(
                    self.remittance_information_unstructured_array.clone()
                        .unwrap_or(vec!["".to_string()]).join(" ")
                );

        let balance_after_transaction = self.balance_after_transaction.clone()
            .unwrap_or(BalanceDto { ..Default::default() });

        let balance_after_transaction_cents = (balance_after_transaction
            .balance_amount.amount
            .parse::<f64>()
            .expect(format!(
                "balance_after_transaction: {}",
                balance_after_transaction.balance_amount.amount
            ).as_str()) * 100f64) as i32;

        let balance_after_transaction_type = balance_after_transaction.balance_type;

        return GoCardlessTransaction {
            transaction_id: self.transaction_id.to_string(),
            booking_date: self.booking_date.to_string(),
            value_date,
            transaction_amount_cents,
            transaction_amount_currency: self.transaction_amount.currency.to_string(),
            creditor_name: self.creditor_name.to_string(),
            debtor_name: self.debtor_name.to_string(),
            debtor_account_iban,
            remittance_information_unstructured,
            balance_after_transaction_cents,
            balance_after_transaction_type,
            internal_transaction_id: self.internal_transaction_id.to_string(),
        };
    }
}