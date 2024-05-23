use diesel::prelude::*;

use crate::establish_db_connection;
use crate::schema::transactions;

pub struct CategoriesService {}

impl CategoriesService {
    pub fn get_categories(&mut self) -> Vec<String> {
        return transactions::table
            .group_by(transactions::category)
            .select(transactions::category)
            .order_by(transactions::category)
            .load::<String>(&mut establish_db_connection())
            .expect("Error loading categories");
    }
}