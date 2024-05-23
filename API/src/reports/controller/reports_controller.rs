use rocket::get;
use rocket::http::Status;
use rocket::response::{content, status};
use crate::guard::ApiKey;
use crate::reports::dto::ReportByCategoryEntryDto;
use crate::reports::service::ReportsService;

#[get("/reports/by_category")]
pub fn get_report_by_category(_key: ApiKey<'_>) -> status::Custom<content::RawJson<String>> {
    let mut reports_service = ReportsService {};

    let mut dtos: Vec<ReportByCategoryEntryDto> = Vec::new();

    for report in reports_service.get_report_by_category() {
        dtos.push(ReportByCategoryEntryDto {
            category: report.category,
            currency: report.currency,
            type_: report.type_,
            total_cents: report.total_cents,
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}
