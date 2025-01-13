use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct CategoryDto {
    pub code: String,
    pub color: String,
    pub discontinued: String,
}
