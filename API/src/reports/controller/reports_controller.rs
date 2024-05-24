use chrono::{Datelike, NaiveDate, Utc};
use rocket::{FromForm, get};
use rocket::http::Status;
use rocket::response::{content, status};
use crate::guard::ApiKey;
use crate::reports::dto::ReportByCategoryEntryDto;
use crate::reports::service::ReportsService;

#[get("/reports/by_category?<params..>")]
pub fn get_report_by_category(
    _key: ApiKey<'_>,
    params: GetReportByCategoryParameters,
) -> status::Custom<content::RawJson<String>> {
    let mut reports_service = ReportsService {};

    let mut dtos: Vec<ReportByCategoryEntryDto> = Vec::new();

    let year = Utc::now().naive_utc().date().year();

    for report in reports_service.get_report_by_category(
        NaiveDate::parse_from_str(&*params.date_from.unwrap_or(format!("{}-01-01", year)), "%Y-%m-%d")
            .expect("Invalid date_from"),
        NaiveDate::parse_from_str(&*params.date_to.unwrap_or(format!("{}-12-31", year)), "%Y-%m-%d")
            .expect("Invalid date_to"),
    ) {
        dtos.push(ReportByCategoryEntryDto {
            category: report.category,
            currency: report.currency,
            total_cents: report.amount_cents,
            transactions_count: report.transactions_count,
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}

#[derive(FromForm)]
pub struct GetReportByCategoryParameters {
    date_from: Option<String>,
    date_to: Option<String>,
}
