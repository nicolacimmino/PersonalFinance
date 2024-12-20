use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct IndicatorReportDto {
    pub date_from: String,
    pub date_to: String,
    pub indicators: Vec::<IndicatorDto>,
}

#[derive(Serialize)]
pub struct IndicatorDto {
    pub label: String,
    pub currency: String,
    pub total_cents: i32,
}
