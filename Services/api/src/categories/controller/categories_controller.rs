use rocket::get;
use rocket::http::Status;
use rocket::response::{content, status};
use crate::categories::dto::CategoryDto;
use crate::categories::service::CategoriesService;
use crate::guard::ApiKey;

#[get("/categories")]
pub fn get_categories(_key: ApiKey<'_>) -> status::Custom<content::RawJson<String>> {
    let mut categories_service = CategoriesService {};

    let mut dtos: Vec<CategoryDto> = Vec::new();

    for category in categories_service.get_categories() {
        dtos.push(CategoryDto {
            code: category.code,
            color: category.color,
            discontinued: category.discontinued
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}
