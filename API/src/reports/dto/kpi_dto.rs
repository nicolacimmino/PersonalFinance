use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct KpiReportDto {
    pub date_from: String,
    pub date_to: String,
    pub kpis: Vec::<KpiReportEntryDto>,
}

#[derive(Serialize)]
pub struct KpiReportEntryDto {
    pub label: String,
    pub currency: String,
    pub total_cents: i32,
}
