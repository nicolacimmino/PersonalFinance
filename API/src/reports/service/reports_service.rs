use chrono::{NaiveDate, NaiveTime};
use diesel::{RunQueryDsl, sql_query};
use diesel::sql_types::{Timestamp};
use rocket::log::private::info;

use crate::establish_db_connection;
use crate::reports::{AggregatedReportByCategoryEntry, ReportByCategoryEntry};
use crate::reports::model::KpiEntry;

pub struct ReportsService {}

impl ReportsService {
    pub fn get_report_by_category(
        &mut self,
        date_from: NaiveDate,
        date_to: NaiveDate,
    ) -> Vec<AggregatedReportByCategoryEntry> {
        let connection = &mut establish_db_connection();

        info!("{}", date_from);
        info!("{}", date_to);

        let reports = sql_query("
            SELECT category,
                    c.type as category_type,
                    c.color,
                    t.currency,
                    CAST(sum(amount_cents) as int4) as amount_cents,
                    CAST(count(*) AS int4) as transactions_count,
                    CAST(sum(amount_cents_eur) as int4) as amount_cents_eur
                       FROM transactions_enriched t
                       INNER JOIN accounts a ON a.id=t.account_id
                       INNER JOIN categories c ON c.code=t.category
                       WHERE t.type<>'TRANSFER' AND t.type<>'INITIAL' AND ( booking_date BETWEEN $1 AND $2 )
                       GROUP BY category, c.type, c.color, t.currency, t.type
          ")
            .bind::<Timestamp, _>(date_from.and_time(NaiveTime::default()))
            .bind::<Timestamp, _>(date_to.and_time(NaiveTime::default()))
            .load::<ReportByCategoryEntry>(connection)
            .expect("Error loading ReportByCategoryEntry");

        let mut categories = reports.iter().map(|category| category.category.to_string())
            .collect::<Vec<_>>();
        categories.sort();
        categories.dedup();

        let mut aggregated_reports: Vec<AggregatedReportByCategoryEntry> = Vec::new();

        for category in categories {
            let mut amount_cents = 0i32;
            let mut transactions_count = 0i32;
            let mut category_color = "";
            let mut category_type = "";

            for report in reports.iter().filter(|&report| report.category == category) {
                amount_cents = amount_cents + report.amount_cents_eur;
                category_color = &report.color;
                category_type = &report.category_type;

                transactions_count += report.transactions_count;
            }


            aggregated_reports.push(AggregatedReportByCategoryEntry {
                category,
                type_: category_type.parse().unwrap(),
                color: category_color.parse().unwrap(),
                currency: "EUR".to_string(),
                amount_cents,
                transactions_count,
            })
        }

        return aggregated_reports;
    }

    pub fn get_kpis(
        &mut self,
        date_from: NaiveDate,
        date_to: NaiveDate,
    ) -> Vec<KpiEntry> {
        let connection = &mut establish_db_connection();

        info!("{}", date_from);
        info!("{}", date_to);

        return  sql_query("
                SELECT * FROM get_kpis($1, $2);
          ")
            .bind::<Timestamp, _>(date_from.and_time(NaiveTime::default()))
            .bind::<Timestamp, _>(date_to.and_time(NaiveTime::default()))
            .load::<KpiEntry>(connection)
            .expect("Error loading KpiEntry");
    }
}