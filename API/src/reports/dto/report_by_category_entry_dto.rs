use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ReportByCategoryDto {
    pub date_from: String,
    pub date_to: String,
    pub reports: Vec::<ReportByCategoryEntryDto>,
}

#[derive(Serialize)]
pub struct ReportByCategoryEntryDto {
    pub category: String,
    #[serde(rename(serialize = "type"))]
    pub type_: String,
    pub currency: String,
    pub total_cents: i32,
    pub transactions_count: i32,
}
