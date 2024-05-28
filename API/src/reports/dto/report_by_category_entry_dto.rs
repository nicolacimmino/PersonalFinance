use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ReportByCategoryEntryDto {
    pub category: String,
    pub currency: String,
    pub total_cents: i32,
    pub transactions_count: i32,
}
