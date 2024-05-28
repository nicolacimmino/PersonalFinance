use std::collections::HashMap;
use bigdecimal::{BigDecimal, ToPrimitive};
use diesel::{PgConnection, RunQueryDsl, sql_query};
use crate::common::ValutaConversionRate;

pub struct ValutaConversionService {
    conversion_factors: HashMap<String, BigDecimal>,
}

impl ValutaConversionService {
    pub fn new(connection: &mut PgConnection) -> Self {
        return ValutaConversionService {
            conversion_factors: Self::get_valuta_rates(connection)
        };
    }

    pub fn convert(&mut self, valuta_from: String, valuta_to: &str, amount_cents: i32) -> i32 {
        let mut factor: BigDecimal = BigDecimal::from(1);
        if valuta_from != valuta_to {
            factor = self.conversion_factors[&*(format!(
                "{}_EUR", valuta_from
            ))].clone()
        }
        return (amount_cents * factor).to_i32().unwrap();
    }

    fn get_valuta_rates(connection: &mut PgConnection) -> HashMap::<String, BigDecimal> {
        let valuta_conversions = sql_query("
           SELECT valuta_from, valuta_to, factor
           FROM valuta_conversion_rates")
            .load::<ValutaConversionRate>(connection)
            .expect("Error loading valuta conversions");

        let mut conversion_factors = HashMap::<String, BigDecimal>::new();

        for valuta_rate in valuta_conversions {
            conversion_factors.insert(
                format!(
                    "{}_{}", valuta_rate.valuta_from, valuta_rate.valuta_to
                ),
                valuta_rate.factor,
            );
        }

        return conversion_factors;
    }
}
