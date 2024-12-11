use chrono::{NaiveDate, NaiveTime};
use diesel::{RunQueryDsl, sql_query};
use diesel::sql_types::{Timestamp};
use rocket::log::private::info;

use crate::establish_db_connection;
use crate::reports::{ReportByCategory};
use crate::reports::model::Indicator;

pub struct ReportsService {}

impl ReportsService {
    pub fn get_report_by_category(
        &mut self,
        date_from: NaiveDate,
        date_to: NaiveDate,
    ) -> Vec<ReportByCategory> {
        let connection = &mut establish_db_connection();

        info!("{}", date_from);
        info!("{}", date_to);

        let reports = sql_query("
                SELECT category,
                       c.type                              as category_type,
                       CAST(count(*) AS int4)              as transactions_count,
                       CAST(sum(amount_cents_eur) as int4) as amount_cents_eur
                FROM application.transactions t
                         INNER JOIN raw.categories c ON c.code = t.category
                WHERE t.type <> 'TRANSFER'
                  AND t.type <> 'INITIAL'
                  AND (booking_date BETWEEN $1 AND $2)
                GROUP BY category, c.type
          ")
            .bind::<Timestamp, _>(date_from.and_time(NaiveTime::default()))
            .bind::<Timestamp, _>(date_to.and_time(NaiveTime::default()))
            .load::<ReportByCategory>(connection)
            .expect("Error loading ReportByCategoryEntry");

        return reports;
    }

    pub fn get_kpis(
        &mut self,
        date_from: NaiveDate,
        date_to: NaiveDate,
    ) -> Vec<Indicator> {
        let connection = &mut establish_db_connection();

        info!("{}", date_from);
        info!("{}", date_to);

        return sql_query("
                SELECT * FROM application.get_indicators($1, $2);
          ")
            .bind::<Timestamp, _>(date_from.and_time(NaiveTime::default()))
            .bind::<Timestamp, _>(date_to.and_time(NaiveTime::default()))
            .load::<Indicator>(connection)
            .expect("Error loading KpiEntry");
    }
}