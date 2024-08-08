use diesel::{Identifiable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::ob_accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ObAccount {
    pub id: Uuid,
    pub provider: String,
    pub provider_account_id: String,
    pub name: String,
    pub req_status: String,
}

