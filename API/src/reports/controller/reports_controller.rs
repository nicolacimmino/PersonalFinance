use chrono::{Datelike, NaiveDate, Utc};
use rocket::{FromForm, get};
use rocket::http::Status;
use rocket::response::{content, status};
use crate::guard::ApiKey;
use crate::reports::dto::{KpiReportDto, KpiReportEntryDto, ReportByCategoryDto, ReportByCategoryEntryDto};
use crate::reports::service::ReportsService;

#[get("/reports/by_category?<params..>")]
pub fn get_report_by_category(
    _key: ApiKey<'_>,
    params: GetReportByCategoryParameters,
) -> status::Custom<content::RawJson<String>> {
    let mut reports_service = ReportsService {};

    let mut report_dtos: Vec<ReportByCategoryEntryDto> = Vec::new();

    let year = Utc::now().naive_utc().date().year();

    let date_from = NaiveDate::parse_from_str(
        &*params.date_from.unwrap_or(format!("{}-01-01", year)), "%Y-%m-%d",
    ).expect("Invalid date_from");

    let date_to = NaiveDate::parse_from_str(
        &*params.date_to.unwrap_or(format!("{}-12-31", year)), "%Y-%m-%d",
    ).expect("Invalid date_to");

    for report in reports_service.get_report_by_category(date_from, date_to) {
        report_dtos.push(ReportByCategoryEntryDto {
            category: report.category,
            type_ : report.category_type,
            currency: "EUR".to_string(),
            total_cents: report.amount_cents_eur,
            transactions_count: report.transactions_count,
        })
    }

    let response_dto = ReportByCategoryDto {
        date_from: date_from.to_string(),
        date_to: date_to.to_string(),
        reports: report_dtos,
    };

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&response_dto).expect("Serialization Failed")),
    )
}

#[derive(FromForm)]
pub struct GetReportByCategoryParameters {
    date_from: Option<String>,
    date_to: Option<String>,
}

#[get("/reports/kpis?<params..>")]
pub fn get_kpis(
    _key: ApiKey<'_>,
    params: GetKpisParameters,
) -> status::Custom<content::RawJson<String>> {
    let mut reports_service = ReportsService {};

    let mut kpis_dtos: Vec<KpiReportEntryDto> = Vec::new();

    let year = Utc::now().naive_utc().date().year();

    let date_from = NaiveDate::parse_from_str(
        &*params.date_from.unwrap_or(format!("{}-01-01", year)), "%Y-%m-%d",
    ).expect("Invalid date_from");

    let date_to = NaiveDate::parse_from_str(
        &*params.date_to.unwrap_or(format!("{}-12-31", year)), "%Y-%m-%d",
    ).expect("Invalid date_to");

    for kpi in reports_service.get_kpis(date_from, date_to) {
        kpis_dtos.push(KpiReportEntryDto {
            label: kpi.label,
            currency: "EUR".to_string(),
            total_cents: kpi.amount_cents,
        })
    }

    let response_dto = KpiReportDto {
        date_from: date_from.to_string(),
        date_to: date_to.to_string(),
        kpis: kpis_dtos,
    };

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&response_dto).expect("Serialization Failed")),
    )
}

#[derive(FromForm)]
pub struct GetKpisParameters {
    date_from: Option<String>,
    date_to: Option<String>,
}

