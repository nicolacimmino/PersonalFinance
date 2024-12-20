use diesel::prelude::*;
use diesel::sql_query;
use crate::categories::model::category::ApplicationCategory;

use crate::establish_db_connection;

pub struct CategoriesService {}

impl CategoriesService {
    pub fn get_categories(&mut self) -> Vec<ApplicationCategory> {
        return sql_query("
            SELECT * FROM application.categories
                ORDER BY code ASC
          ").load::<ApplicationCategory>(&mut establish_db_connection())
            .expect("Error loading categories");
    }
}