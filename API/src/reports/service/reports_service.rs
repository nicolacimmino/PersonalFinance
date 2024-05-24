use std::collections::HashMap;

use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{NaiveDate, NaiveTime};
use diesel::{PgConnection, RunQueryDsl, sql_query};
use diesel::sql_types::{Timestamp};
use rocket::log::private::info;

use crate::common::ValutaConversionRate;
use crate::establish_db_connection;
use crate::reports::{AggregatedReportByCategoryEntry, ReportByCategoryEntry};

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

        let conversion_factors = self.get_valuta_rates(connection);

        let reports = sql_query("
           SELECT category, currency, CAST(sum(amount_cents) as int8) as amount_cents, CAST(count(*) AS int8) as transactions_count
           FROM transactions t
           INNER JOIN accounts a ON a.id=t.account_id
           WHERE t.type<>'TRANSFER' AND t.type<>'INITIAL' AND ( booking_date BETWEEN $1 AND $2 )
           GROUP BY category, currency, type")
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
            let mut amount_cents = 0i64;
            let mut transactions_count = 0i64;
            for report in reports.iter().filter(|&report| report.category == category) {
                let mut factor: BigDecimal = BigDecimal::from(1);
                if report.currency != "EUR" {
                    factor = conversion_factors[&*(format!(
                        "{}_EUR", report.currency
                    ))].clone()
                }
                amount_cents = amount_cents + (report.amount_cents * factor).to_i64().unwrap();
                transactions_count += report.transactions_count;
            }

            aggregated_reports.push(AggregatedReportByCategoryEntry {
                category,
                currency: "EUR".to_string(),
                amount_cents,
                transactions_count,
            })
        }

        return aggregated_reports;
    }

    fn get_valuta_rates(&mut self, connection: &mut PgConnection) -> HashMap::<String, BigDecimal> {
        let valuta_conversions = sql_query("
           SELECT valuta_from, valuta_to, factor
           FROM valuta_conversion_rates")
            .load::<ValutaConversionRate>(connection)
            .expect("Error loading valuta conversions");

        let mut conversion_factors = HashMap::<String, BigDecimal>::new();

        for valuta_rate in valuta_conversions {
            conversion_factors.insert(
                format!(
                    "{}_{}", valuta_rate.valuta_from, valuta_rate.valuta_to
                ),
                valuta_rate.factor,
            );
        }

        return conversion_factors;
    }
}