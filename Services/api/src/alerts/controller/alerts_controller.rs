use rocket::get;
use rocket::http::Status;
use rocket::response::{content, status};

use crate::alerts::dto::AlertDto;
use crate::alerts::service::AlertsService;
use crate::guard::ApiKey;

#[get("/alerts")]
pub fn get_alerts(_key: ApiKey<'_>) -> status::Custom<content::RawJson<String>> {
    let mut alerts_service = AlertsService {};

    let mut dtos: Vec<AlertDto> = Vec::new();

    for alert in alerts_service.get_alerts() {
        dtos.push(AlertDto {
            level: alert.level.to_owned(),
            category: alert.category.to_owned(),
            message: alert.message.to_owned(),
            item: alert.item.to_owned(),
            item_id: alert.item_id.to_owned(),
        })
    }

    status::Custom(Status::Ok, content::RawJson(
        serde_json::to_string(&dtos).expect("Serialization Failed")),
    )
}
