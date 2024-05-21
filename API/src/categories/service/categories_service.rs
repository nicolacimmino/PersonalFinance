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

/*
    Stash for now, we can actually query, aggregate and build types that are not
    db tables. We keep it simple for now. This should be a /categories/<id>/reports
   return transactions::table
            .inner_join(accounts::table)
            .group_by((
                transactions::category,
                transactions::type_,
                accounts::currency
            ))
            .select((
                transactions::category,
                transactions::type_,
                accounts::currency,
                diesel::dsl::sum(transactions::amount_cents)
            ))
            .load::<Category>(&mut establish_db_connection())
            .expect("Error loading categories");

 */