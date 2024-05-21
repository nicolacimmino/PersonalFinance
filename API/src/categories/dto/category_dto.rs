use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct CategoryDto {
    pub category: String,
    #[serde(rename(serialize = "type"))]
    pub type_: String,
}
