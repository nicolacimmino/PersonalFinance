use diesel::prelude::*;
use crate::categories::Category;

use crate::establish_db_connection;
use crate::schema::transactions;

pub struct CategoriesService {}

impl CategoriesService {
    pub fn get_categories(&mut self) -> Vec<Category> {
        return transactions::table
            .group_by((transactions::category, transactions::type_))
            .select((transactions::category, transactions::type_))
            .load::<Category>(&mut establish_db_connection())
            .expect("Error loading categories");
    }
}