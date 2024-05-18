use diesel::{Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::ob_accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ObAccount {
    pub id: Uuid,
    pub provider: String,
    pub provider_account_id: String,
    pub name: String
}

