use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AccountDto {
    pub iban: String,
}


impl Default for AccountDto {
    fn default() -> AccountDto {
        return AccountDto {
            iban: "".to_string()
        };
    }
}