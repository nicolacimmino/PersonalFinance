use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::ob_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewObTransaction<'a> {
    pub transaction_id: &'a str,
    pub booking_date: &'a str,
    pub value_date: &'a str,
    pub booking_date_time: &'a str,
    pub transaction_amount_cents: i32,
    pub transaction_amount_currency: &'a str,
    pub creditor_name: &'a str,
    pub debtor_name: &'a str,
    pub debtor_account_iban: &'a str,
    pub remittance_information_unstructured: &'a str,
    pub balance_after_transaction_amount_cents: i32,
    pub balance_after_transaction_currency: &'a str,
    pub balance_after_transaction_type: &'a str,
    pub internal_transaction_id: &'a str,
}
