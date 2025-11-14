use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Mint not initialezed")]
    MintNotInitialezed,
    #[msg("Token account is frozen")]
    TokenAccountFrozen,
    #[msg("Mint authority is present; supply is mutable")]
    MintAuthorityPresent,
    #[msg("Mint mismatch between token account and provided mint")]
    MintMismatch,
    #[msg("The provided signer is not the owner of the token account")]
    NotAccountOwner,
    #[msg("Amount must be positive")]
    AmountMustBePositive,
    #[msg("Insufficient funds in token account")]
    InsufficientFunds,
    #[msg("Wrong Token Program for provided mint/token account")]
    WrongTokenProgram,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Message too long (max 256 bytes)")]
    MessageTooLong,
    #[msg("Bad fee recipient passed by client")]
    BadFeeRecipient,
    #[msg("Missing fee accounts by client")]
    MissingFeeAccounts,
    #[msg("Unauthorized admin")]
    Unauthorized,
}