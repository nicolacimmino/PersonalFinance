use crate::go_cardless::dto::prelude::AccountInfoResponseDto;

pub struct GoCardlessAccountInfo {
    pub status: String,
    pub iban: String,
}

pub trait ConvertsToGoCardlessAccountInfo {
    fn to_gocardless_account_info(&self) -> GoCardlessAccountInfo;
}

impl ConvertsToGoCardlessAccountInfo for &AccountInfoResponseDto {
    fn to_gocardless_account_info(&self) -> GoCardlessAccountInfo {
        let status = match self.status.as_str() {
            "READY" => "READY",
            "EXPIRED" => "EXPIRED",
            _ => "ERROR",
        };

        return GoCardlessAccountInfo {
            status: (&status).parse().unwrap(),
            iban: self.iban.clone(),
        };
    }
}