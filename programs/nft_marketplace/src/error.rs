use anchor_lang::error_code;

#[error_code]
pub enum MarketPlaceError {
    #[msg("Given name is too long...")]
    NameTooLong,

    #[msg("NFT Collection is not valid")]
    InvalidCollection,
}
