use diesel::{RunQueryDsl, sql_query};
use crate::establish_db_connection;
use crate::reports::ReportByCategoryEntry;

pub struct ReportsService {}

impl ReportsService {
    pub fn get_report_by_category(&mut self) -> Vec<ReportByCategoryEntry> {
        return sql_query("
           SELECT category, currency, type as type_, sum(amount_cents) as total_cents
           FROM transactions t
           INNER JOIN accounts a ON a.id=t.account_id
            GROUP BY category, currency, type
        ").load::<ReportByCategoryEntry>(&mut establish_db_connection())
            .expect("Error loading ReportByCategoryEntry");
    }
}