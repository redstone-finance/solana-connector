use anchor_lang::prelude::*;

#[error_code]
pub enum RedstoneError {
    #[msg("Size not supported")]
    SizeNotSupported,

    #[msg("Number overflow")]
    NumberOverflow,

    #[msg("Invalid RedStone marker")]
    InvalidRedstoneMarker,

    #[msg("Invalid signature")]
    InvalidSignature,

    #[msg("Timestamp is too old")]
    TimestampTooOld,

    #[msg("Timestamp is too far in the future")]
    TimestampTooFuture,

    #[msg("Insufficient number of unique signers")]
    InsufficientSignerCount,

    #[msg("Failed to parse payload")]
    PayloadParseError,

    #[msg("Invalid data package")]
    InvalidDataPackage,

    #[msg("Unsupported feed ID")]
    UnsupportedFeedId,

    #[msg("Price account missing")]
    MissingPriceAccount,

    #[msg("Failed to calculate median")]
    MedianCalculationError,

    #[msg("Packages need to have the same timestamp")]
    TimestampMismatch,
}
