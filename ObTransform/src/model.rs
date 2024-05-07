use chrono::NaiveDateTime;
use diesel::{Associations, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub code: String,
    pub description: String,
    pub currency: String,
}

#[derive(Queryable, Selectable, Associations, Debug)]
#[diesel(belongs_to(Account, foreign_key = account_id))]
#[diesel(table_name = crate::schema::ob_accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ObAccount {
    pub id: Uuid,
    pub provider: String,
    pub provider_account_id: String,
    pub name: String,
    pub account_id: i32,
}

#[derive(Queryable, Selectable, Associations, Debug)]
#[diesel(belongs_to(ObAccount, foreign_key = ob_account_id))]
#[diesel(table_name = crate::schema::ob_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ObTransaction {
    pub id: Uuid,
    pub ob_account_id: Uuid,
    pub transaction_id: String,
    pub booking_date: String,
    pub value_date: String,
    pub booking_date_time: String,
    pub transaction_amount_cents: i32,
    pub transaction_amount_currency: String,
    pub creditor_name: String,
    pub debtor_name: String,
    pub debtor_account_iban: String,
    pub remittance_information_unstructured: String,
    pub balance_after_transaction_amount_cents: i32,
    pub balance_after_transaction_currency: String,
    pub balance_after_transaction_type: String,
    pub internal_transaction_id: String,
}


#[derive(Queryable, PartialEq, Selectable, Debug)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transaction {
    pub id: i32,
    pub date: NaiveDateTime,
    pub type_: String,
    pub account_id: i32,
    pub amount_cents: i32,
    pub category: String,
    pub creditor_name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTransaction<'a> {
    pub date: &'a NaiveDateTime,
    pub type_: &'a str,
    pub account_id: i32,
    pub amount_cents: i32,
    pub category: &'a str,
    pub creditor_name: &'a str,
    pub description: &'a str,
}
