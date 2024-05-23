use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ReportByCategoryEntryDto {
    pub category: String,
    pub currency: String,
    pub type_: String,
    pub total_cents: i64,
}
