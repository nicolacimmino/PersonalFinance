mod account_dto;
mod amount_dto;
mod balance_dto;
mod create_token_response_dto;
mod create_token_request_dto;
mod get_transactions_response_dto;
mod transaction_dto;

pub(crate) mod prelude {
    pub use crate::go_cardless::dto::account_dto::AccountDto;
    pub use crate::go_cardless::dto::balance_dto::BalanceDto;
    pub use crate::go_cardless::dto::create_token_request_dto::CreateTokenRequestDto;
    pub use crate::go_cardless::dto::create_token_response_dto::CreateTokenResponseDto;
    pub use crate::go_cardless::dto::get_transactions_response_dto::GetTransactionsResponseDto;
    pub use crate::go_cardless::dto::transaction_dto::TransactionDto;
}
