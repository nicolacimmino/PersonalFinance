use diesel::{QueryableByName, Selectable};

#[derive(QueryableByName, Selectable, Debug)]
#[diesel(table_name = crate::manual_schema::application_accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ApplicationAccount {
    pub id: i32,
    pub code: String,
    pub description: String,
    pub currency: String,
    pub iban: String,
    pub status: String,
    pub asset_type: String,
    pub pri_transactions_src: String,
    pub balance_cents: i32,
    pub balance_eur_cents: i32,
}