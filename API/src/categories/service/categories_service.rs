use diesel::prelude::*;
use crate::categories::model::category::Category;

use crate::establish_db_connection;
use crate::schema::categories;

pub struct CategoriesService {}

impl CategoriesService {
    pub fn get_categories(&mut self) -> Vec<Category> {
        return categories::table
            .select(Category::as_select())
            .order(categories::code.asc())
            .load::<Category>(&mut establish_db_connection())
            .expect("Error loading categories");
    }
}