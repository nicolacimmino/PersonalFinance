use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, ExpressionMethods};
use crate::schema;
use schema::ob_accounts::dsl::ob_accounts;
use crate::open_banking::ObAccount;

pub struct AccountsService<'a> {
    connection: &'a mut PgConnection,
}

impl AccountsService<'_> {
    pub fn new(connection: &mut PgConnection) -> AccountsService {
        return AccountsService {
            connection
        };
    }
    pub fn get_all_accounts(&mut self) -> Vec<ObAccount> {
        return ob_accounts
            .filter(schema::ob_accounts::provider.eq("GOCARDLESS"))
            .select(ObAccount::as_select())
            .load(self.connection)
            .expect("Error loading ob_accounts");
    }
}