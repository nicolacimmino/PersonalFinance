use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct AlertDto {
    pub category: String,
    pub message: String,
    pub item: String,
    pub item_id: String,
    pub level: String,
}
