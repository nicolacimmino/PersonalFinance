use chrono::NaiveDateTime;
use diesel::{Associations, Insertable, Queryable, Selectable};
use uuid::Uuid;


#[derive(Queryable, Selectable, Associations, Debug)]
#[diesel(belongs_to(Account, foreign_key = account_id))]
#[diesel(table_name = crate::schema::sp_accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpAccount {
    pub id: Uuid,
    pub wallet: String,
    pub account_id: Option<i32>,
}

#[derive(Queryable, PartialEq, Selectable, Associations, Debug)]
#[diesel(belongs_to(SpAccount, foreign_key = sp_account_id))]
#[diesel(table_name = crate::schema::sp_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpTransaction {
    pub id: Uuid,
    pub date: String,
    pub wallet: String,
    pub type_: String,
    pub category: String,
    pub amount: String,
    pub currency: String,
    pub note: String,
    pub labels: String,
    pub author: String,
    pub transformed_transaction_id: Option<i32>,
    pub sp_account_id: Option<Uuid>
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub code: String,
    pub description: String,
    pub currency: String,
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